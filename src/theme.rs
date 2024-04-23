use serde::Deserialize;

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	Dark,
	#[default]
	Light,
}

impl Theme {
	#[cfg(feature = "nightly")]
	pub fn get_icon_bytes(&self) -> Vec<u8> {
		include_bytes!("../assets/ico_nightly/32-32.png").to_vec()
	}

	#[cfg(not(feature = "nightly"))]
	pub fn get_icon_bytes(&self) -> Vec<u8> {
		include_bytes!("../assets/ico/32-32.png").to_vec()
	}

	#[cfg(feature = "nightly")]
	pub fn get_logo_bytes(&self) -> Vec<u8> {
		include_bytes!("../assets/nightly-logo.png").to_vec()
	}

	#[cfg(not(feature = "nightly"))]
	pub fn get_logo_bytes(&self) -> Vec<u8> {
		match self {
			Theme::Dark => include_bytes!("../assets/cnil-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/cnil-logo.png").to_vec(),
		}
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
