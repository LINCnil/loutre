use crate::config::Config;
use dioxus::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	Dark,
	#[default]
	Light,
}

impl fmt::Display for Theme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let theme = match &self {
			Self::Dark => "dark",
			Self::Light => "light",
		};
		write!(f, "{theme}")
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseThemeError;

impl FromStr for Theme {
	type Err = ParseThemeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"dark" => Ok(Self::Dark),
			"light" => Ok(Self::Light),
			_ => Err(ParseThemeError),
		}
	}
}

pub async fn get_default_theme() -> Theme {
	let js =
		"return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;";
	let rsp = eval(js).await.unwrap();
	if let Some(is_dark) = rsp.as_bool() {
		if is_dark {
			return Theme::Dark;
		}
	}
	Theme::Light
}

pub async fn set_theme(theme: Theme) {
	// Set the theme context
	let js = format!("document.documentElement.setAttribute('data-theme', '{theme}');");
	let _ = eval(&js).await;
	let mut theme_sig = use_context::<Signal<Theme>>();
	theme_sig.set(theme);

	// Write it to the configuration
	let mut config_sig = use_context::<Signal<Config>>();
	let mut config = config_sig();
	config.theme = Some(theme);
	config.write_to_file();
	config_sig.set(config);
}
