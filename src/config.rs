use crate::content_file::ContentFileFormat;
use crate::hash::HashFunc;
use crate::i18n::Lang;
use crate::nb_repr::NbRepr;
use crate::theme::Theme;
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
	pub theme: Option<Theme>,
	pub lang: Lang,
	pub content_file_name: Option<String>,
	pub content_file_format: ContentFileFormat,
	pub number_representation: NbRepr,
	pub hash_function: HashFunc,
	pub clipboard_persistence: Option<bool>,
	pub clipboard_threshold: Option<usize>,
	pub enable_duplicate_file_warning: Option<bool>,
	pub enable_empty_file_warning: Option<bool>,
	pub include_hidden_files: Option<bool>,
	pub include_system_files: Option<bool>,
}

impl Config {
	pub fn init() -> Self {
		let path = Config::get_file_path();
		let ctn = if path.is_file() {
			read_to_string(&path).unwrap_or_default()
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

	pub fn get_content_file_name(&self) -> String {
		match &self.content_file_name {
			Some(name) => name.to_string(),
			None => self
				.content_file_format
				.default_content_file_name(self.hash_function),
		}
	}

	pub fn get_clipboard_threshold(&self) -> usize {
		self.clipboard_threshold
			.unwrap_or(crate::DEFAULT_CLIPBOARD_THRESHOLD)
	}

	pub fn is_duplicate_file_warning_enabled(&self) -> bool {
		self.enable_duplicate_file_warning.unwrap_or(true)
	}

	pub fn is_empty_file_warning_enabled(&self) -> bool {
		self.enable_empty_file_warning.unwrap_or(true)
	}

	pub fn include_hidden_files(&self) -> bool {
		self.include_hidden_files.unwrap_or(false)
	}

	pub fn include_system_files(&self) -> bool {
		self.include_system_files.unwrap_or(false)
	}

	fn load_config(content: &str) -> Config {
		toml::from_str(content).unwrap_or_default()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use unic_langid::langid;

	#[test]
	fn test_config() {
		let s = r#"
theme = "dark"
lang = "fr"
number_representation = "letters"
content_file_format = "cksum-gnu"
hash_function = "sha-512"
"#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Some(Theme::Dark));
		assert_eq!(cfg.lang, langid!("fr").into());
		assert_eq!(cfg.number_representation, NbRepr::Letters);
		assert_eq!(cfg.hash_function, HashFunc::Sha512);
		assert_eq!(cfg.content_file_name, None);
		assert_eq!(cfg.content_file_format, ContentFileFormat::CksumGnu);
		assert_eq!(cfg.get_content_file_name(), "sha512sums.txt".to_string());
	}

	#[test]
	fn test_config_cnil() {
		let s = r#"
theme = "dark"
lang = "fr"
number_representation = "letters"
content_file_format = "cnil"
hash_function = "sha-256"
"#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Some(Theme::Dark));
		assert_eq!(cfg.lang, langid!("fr").into());
		assert_eq!(cfg.number_representation, NbRepr::Letters);
		assert_eq!(cfg.hash_function, HashFunc::Sha256);
		assert_eq!(cfg.content_file_name, None);
		assert_eq!(cfg.content_file_format, ContentFileFormat::Cnil);
		assert_eq!(cfg.get_content_file_name(), "contenu.txt".to_string());
	}

	#[test]
	fn test_empty_config() {
		let cfg = Config::load_config("");
		assert_eq!(cfg.theme, None);
		assert_eq!(cfg.lang, Lang::default());
		assert_eq!(cfg.number_representation, NbRepr::default());
		assert_eq!(cfg.hash_function, HashFunc::default());
		assert_eq!(cfg.content_file_name, None);
		assert_eq!(cfg.content_file_format, ContentFileFormat::default());
		assert_eq!(
			cfg.get_content_file_name(),
			"CHECKSUM.SHA256.txt".to_string()
		);
	}

	#[test]
	fn test_invalid_config() {
		let s = r#"
theme = "dark invalid theme"
lang = "not a valid language tag"
content_file_format = "also invalid"
number_representation = "still invalid"
hash_function = "also invalid"
"#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, None);
		assert_eq!(cfg.lang, Lang::default());
		assert_eq!(cfg.number_representation, NbRepr::default());
		assert_eq!(cfg.hash_function, HashFunc::default());
		assert_eq!(cfg.content_file_name, None);
		assert_eq!(cfg.content_file_format, ContentFileFormat::default());
		assert_eq!(
			cfg.get_content_file_name(),
			"CHECKSUM.SHA256.txt".to_string()
		);
	}
}
