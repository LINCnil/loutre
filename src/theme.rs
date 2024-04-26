use eframe::egui::Color32;
use serde::Deserialize;

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	Dark,
	#[default]
	Light,
	#[cfg(feature = "nightly")]
	Nightly,
}

impl Theme {
	pub fn get_info_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0x7a, 0xcb, 0xff),
			Theme::Light => Color32::from_rgb(0x7a, 0xcb, 0xff),
			#[cfg(feature = "nightly")]
			Theme::Nightly => Color32::from_rgb(0x7a, 0xcb, 0xff),
		}
	}

	pub fn get_success_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0xe7, 0xf7, 0xed),
			Theme::Light => Color32::from_rgb(0xe7, 0xf7, 0xed),
			#[cfg(feature = "nightly")]
			Theme::Nightly => Color32::from_rgb(0xe7, 0xf7, 0xed),
		}
	}

	pub fn get_warning_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0xff, 0xeb, 0x3e),
			Theme::Light => Color32::from_rgb(0xff, 0xeb, 0x3e),
			#[cfg(feature = "nightly")]
			Theme::Nightly => Color32::from_rgb(0xff, 0xeb, 0x3e),
		}
	}

	pub fn get_icon_bytes(&self) -> Vec<u8> {
		match self {
			#[cfg(feature = "nightly")]
			Theme::Nightly => include_bytes!("../assets/ico_nightly/32-32.png").to_vec(),
			_ => include_bytes!("../assets/ico/32-32.png").to_vec(),
		}
	}

	pub fn get_logo_bytes(&self) -> Vec<u8> {
		match self {
			Theme::Dark => include_bytes!("../assets/cnil-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/cnil-logo.png").to_vec(),
			#[cfg(feature = "nightly")]
			Theme::Nightly => include_bytes!("../assets/nightly-logo.png").to_vec(),
		}
	}
}

impl From<Theme> for eframe::Theme {
	fn from(t: Theme) -> Self {
		match t {
			Theme::Dark => eframe::Theme::Dark,
			Theme::Light => eframe::Theme::Light,
			#[cfg(feature = "nightly")]
			Theme::Nightly => eframe::Theme::Light,
		}
	}
}
