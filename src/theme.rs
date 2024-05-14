use crate::i18n::I18n;
use eframe::egui::{self, Color32, RichText};
use serde::{Deserialize, Serialize};

pub const AVAILABLE_THEMES: &[Theme] = &[Theme::Dark, Theme::Light];
const SIGN_INFO: &str = "ℹ";
const SIGN_SUCCESS: &str = "✔";
const SIGN_WARNING: &str = "⚠";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	Dark,
	#[default]
	Light,
	#[cfg(feature = "nightly")]
	#[serde(rename(serialize = "dark"))]
	NightlyDark,
	#[cfg(feature = "nightly")]
	#[serde(rename(serialize = "light"))]
	NightlyLight,
}

impl Theme {
	pub fn get_icon_bytes(&self) -> Vec<u8> {
		match self {
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => {
				include_bytes!("../assets/ico_nightly/32-32.png").to_vec()
			}
			_ => include_bytes!("../assets/ico/32-32.png").to_vec(),
		}
	}

	pub fn get_logo_bytes(&self) -> Vec<u8> {
		match self {
			Theme::Dark => include_bytes!("../assets/cnil-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/cnil-logo.png").to_vec(),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => {
				include_bytes!("../assets/nightly-logo.png").to_vec()
			}
		}
	}

	pub fn display(&self, i18n: &I18n) -> String {
		match self {
			Theme::Dark => i18n.msg("theme_dark"),
			Theme::Light => i18n.msg("theme_light"),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => i18n.msg("theme_nightly"),
		}
	}

	fn add_label<F>(&self, ui: &mut egui::Ui, text: &str, icon: &str, color: &Color32, extra: F)
	where
		F: Fn(&mut egui::Ui),
	{
		let margin = egui::Margin::from(6.0);
		egui::Frame::none()
			.inner_margin(margin)
			.fill(*color)
			.show(ui, |ui| {
				ui.horizontal(|ui| {
					ui.label(RichText::new(icon).size(20.0));
					ui.add(egui::Label::new(text).wrap(true));
					extra(ui);
					ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |_ui| {});
				});
			});
	}

	pub fn add_info_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_info_label_extra(ui, text, |_| {});
	}

	pub fn add_info_label_extra<F>(&self, ui: &mut egui::Ui, text: &str, extra: F)
	where
		F: Fn(&mut egui::Ui),
	{
		self.add_label(ui, text, SIGN_INFO, &self.get_info_label_bg(), extra);
	}

	pub fn add_success_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_label(ui, text, SIGN_SUCCESS, &self.get_success_label_bg(), |_| {});
	}

	pub fn add_warning_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_label(ui, text, SIGN_WARNING, &self.get_warning_label_bg(), |_| {});
	}

	fn get_info_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0x7a, 0xcb, 0xff),
			Theme::Light => Color32::from_rgb(0x7a, 0xcb, 0xff),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => Color32::from_rgb(0x7a, 0xcb, 0xff),
		}
	}

	fn get_success_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0xe7, 0xf7, 0xed),
			Theme::Light => Color32::from_rgb(0xe7, 0xf7, 0xed),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => Color32::from_rgb(0xe7, 0xf7, 0xed),
		}
	}

	fn get_warning_label_bg(&self) -> Color32 {
		match self {
			Theme::Dark => Color32::from_rgb(0xff, 0xeb, 0x3e),
			Theme::Light => Color32::from_rgb(0xff, 0xeb, 0x3e),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => Color32::from_rgb(0xff, 0xeb, 0x3e),
		}
	}
}

impl From<Theme> for eframe::Theme {
	fn from(t: Theme) -> Self {
		match t {
			Theme::Dark => eframe::Theme::Dark,
			Theme::Light => eframe::Theme::Light,
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => eframe::Theme::Light,
		}
	}
}
