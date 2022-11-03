use crate::theme::Theme;
use serde_derive::Deserialize;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Default, Deserialize)]
pub struct Config {
	pub theme: Theme,
}

impl Config {
	pub fn init() -> Self {
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
		let ctn = if !path.is_file() {
			if let Ok(mut f) = File::create(&path) {
				let _ = f.write_all(crate::DEFAULT_CONFIG.as_bytes());
			}
			crate::DEFAULT_CONFIG.to_string()
		} else {
			match read_to_string(&path) {
				Ok(ctn) => ctn,
				Err(_) => crate::DEFAULT_CONFIG.to_string(),
			}
		};
		Config::load_config(&ctn)
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
	use crate::theme::Theme;

	#[test]
	fn test_config() {
		let s = r#"theme = "dark""#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Theme::Dark);
	}

	#[test]
	fn test_empty_config() {
		let cfg = Config::load_config("");
		assert_eq!(cfg.theme, Theme::default());
	}

	#[test]
	fn test_invalid_config() {
		let s = r#"theme = "dark invalid theme""#;
		let cfg = Config::load_config(s);
		assert_eq!(cfg.theme, Theme::default());
	}
}
