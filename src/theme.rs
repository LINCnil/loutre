use serde_with::DeserializeFromStr;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr)]
pub enum Theme {
	Dark,
	Light,
}

impl Theme {
	pub fn get_icon_bytes(&self) -> Vec<u8> {
		include_bytes!("../assets/ico/32-32.png").to_vec()
	}

	pub fn get_logo_bytes(&self) -> Vec<u8> {
		match self {
			Theme::Dark => include_bytes!("../assets/cnil-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/cnil-logo.png").to_vec(),
		}
	}
}

impl Default for Theme {
	fn default() -> Self {
		Theme::Light
	}
}

impl FromStr for Theme {
	type Err = u8;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"dark" => Theme::Dark,
			"light" => Theme::Light,
			_ => Theme::default(),
		})
	}
}

impl From<Theme> for eframe::Theme {
	fn from(t: Theme) -> Self {
		match t {
			Theme::Dark => eframe::Theme::Dark,
			Theme::Light => eframe::Theme::Light,
		}
	}
}
