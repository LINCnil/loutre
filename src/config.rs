use crate::hasher::HashFunc;
use crate::i18n::I18n;
use crate::nb_repr::NbRepr;
use crate::theme::Theme;
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
	pub theme: Theme,
	pub lang: String,
	pub number_representation: NbRepr,
	pub content_file_name: Option<String>,
	pub hash_function: HashFunc,
	pub clipboard_persistence: Option<bool>,
}

impl Config {
	pub fn init() -> Self {
		let path = Config::get_file_path();
		let ctn = if path.is_file() {
			match read_to_string(&path) {
				Ok(ctn) => ctn,
				Err(_) => String::new(),
			}
		} else {
			String::new()
		};
		Config::load_config(&ctn)
	}

	pub fn write_to_file(&self) {
		let path = Config::get_file_path();
		if let Ok(mut f) = File::create(path) {
			let ctn = toml::to_string(&self).unwrap();
			let _ = f.write_all(ctn.as_bytes());
		}
	}

	fn get_file_path() -> PathBuf {
		let mut path = match dirs::config_dir() {
			Some(p) => p,
			None => PathBuf::new(),
		};
		path.push(crate::CONFIG_FILE_DIR);
		path.push(crate::CONFIG_FILE_SUBDIR);
		if !path.is_dir() {
			let _ = create_dir_all(&path);
		}
		path.push(crate::CONFIG_FILE_NAME);
		path
	}

	pub fn content_file_name(&self, i18n: &I18n) -> String {
		match &self.content_file_name {
			Some(name) => {
				if name.is_empty() {
					i18n.msg("content_file_name")
				} else {
					name.to_owned()
				}
			}
			None => i18n.msg("content_file_name"),
		}
	}

	fn load_config(content: &str) -> Config {
		match toml::from_str(content) {
			Ok(cfg) => cfg,
			Err(_) => Config::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Config;
	use crate::nb_repr::NbRepr;
	use crate::theme::Theme;

	#[test]
	fn test_config() {
		let s = r#"
theme = "dark"
lang = "fr"
number_representation = "letters"
content_file_name = "test"
"#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Theme::Dark);
		assert_eq!(&cfg.lang, "fr");
		assert_eq!(cfg.number_representation, NbRepr::Letters);
		assert_eq!(cfg.content_file_name, Some("test".to_string()));
	}

	#[test]
	fn test_empty_config() {
		let cfg = Config::load_config("");
		assert_eq!(cfg.theme, Theme::default());
		assert_eq!(cfg.lang, String::new());
	}

	#[test]
	fn test_invalid_config() {
		let s = r#"
theme = "dark invalid theme"
lang = "not a valid language tag"
number_representation = "still invalid"
"#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Theme::default());
		assert_eq!(cfg.lang, String::new());
	}
}
