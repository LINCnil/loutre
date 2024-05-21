mod button;
mod color;
mod icon;
mod infobox;

pub use button::Button;
pub use color::Color;
pub use icon::Icon;
pub use infobox::{InfoBox, InfoBoxLevel, InfoBoxType};

use crate::i18n::I18n;
use eframe::egui::{self, FontFamily, FontId, RichText, TextStyle};
use serde::{Deserialize, Serialize};

pub const AVAILABLE_THEMES: &[Theme] = &[Theme::Dark, Theme::Light];
pub const MAIN_ROUNDING: f32 = 7.0;

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
	pub fn set_fonts(&self, ctx: &egui::Context) {
		let mut fonts = egui::FontDefinitions::default();

		// OpenSans-Bold
		fonts.font_data.insert(
			"OpenSans-Bold".to_owned(),
			egui::FontData::from_static(include_bytes!("../assets/fonts/OpenSans-Bold.ttf")),
		);
		fonts
			.families
			.entry(FontFamily::Name("bold".into()))
			.or_default()
			.insert(0, "OpenSans-Bold".to_owned());

		// OpenSans-Regular
		fonts.font_data.insert(
			"OpenSans-Regular".to_owned(),
			egui::FontData::from_static(include_bytes!("../assets/fonts/OpenSans-Regular.ttf")),
		);
		fonts
			.families
			.entry(FontFamily::Proportional)
			.or_default()
			.insert(0, "OpenSans-Regular".to_owned());
		fonts
			.families
			.entry(FontFamily::Name("icon".into()))
			.or_default()
			.push("OpenSans-Regular".to_owned());

		// RemixIcon
		fonts.font_data.insert(
			"RemixIcon".to_owned(),
			egui::FontData::from_static(include_bytes!("../assets/fonts/remixicon.ttf")),
		);
		fonts
			.families
			.entry(FontFamily::Name("icon".into()))
			.or_default()
			.insert(0, "RemixIcon".to_owned());

		ctx.set_fonts(fonts);

		let mut style = (*ctx.style()).clone();
		style.text_styles = [
			(
				TextStyle::Heading,
				FontId::new(16.0, FontFamily::Name("bold".into())),
			),
			(TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
			(
				TextStyle::Button,
				FontId::new(16.0, FontFamily::Proportional),
			),
			(TextStyle::Small, FontId::new(8.0, FontFamily::Proportional)),
			(
				TextStyle::Name("icon".into()),
				FontId::new(16.0, FontFamily::Name("icon".into())),
			),
		]
		.into();
		ctx.set_style(style);
	}

	pub fn icon(&self, icon: char) -> RichText {
		RichText::new(icon).text_style(TextStyle::Name("icon".into()))
	}

	pub fn icon_with_txt(&self, icon: char, text: &str) -> RichText {
		RichText::new(format!("{icon} {text}")).text_style(TextStyle::Name("icon".into()))
	}

	pub fn get_icon_bytes(&self) -> Vec<u8> {
		match self {
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => {
				include_bytes!("../assets/ico_nightly/32-32.png").to_vec()
			}
			_ => include_bytes!("../assets/ico/32-32.png").to_vec(),
		}
	}

	pub fn get_logo_bytes(&self) -> (String, Vec<u8>) {
		match self {
			Theme::Dark => (
				"bytes://logo-dark".to_string(),
				include_bytes!("../assets/main-logo-dark.png").to_vec(),
			),
			Theme::Light => (
				"bytes://logo-light".to_string(),
				include_bytes!("../assets/main-logo-light.png").to_vec(),
			),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => (
				"bytes://logo-nightly".to_string(),
				include_bytes!("../assets/main-logo-nightly.png").to_vec(),
			),
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

	pub fn get_main_frame(&self) -> egui::Frame {
		egui::Frame::default()
			.inner_margin(8.0)
			.fill(Color::MainFrameBackground.get(*self))
	}

	pub fn set_visuals(&self, visuals: &mut egui::style::Visuals) {
		// See also:
		// https://docs.rs/egui/latest/egui/style/struct.Visuals.html
		// https://docs.rs/egui/latest/egui/style/struct.Widgets.html
		// https://docs.rs/egui/latest/egui/style/struct.WidgetVisuals.html

		// Widgets (default)
		visuals.widgets.inactive.bg_fill = Color::ButtonBackground.get(*self);
		visuals.widgets.inactive.weak_bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.inactive.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorder.get(*self),
		};
		visuals.widgets.inactive.rounding = MAIN_ROUNDING.into();
		visuals.widgets.inactive.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonText.get(*self),
		};

		// Widgets (hovered)
		visuals.widgets.hovered.bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.hovered.weak_bg_fill = Color::ButtonBackgroundHovered.get(*self);
		visuals.widgets.hovered.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorderHovered.get(*self),
		};
		visuals.widgets.hovered.rounding = MAIN_ROUNDING.into();
		visuals.widgets.hovered.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonTextHovered.get(*self),
		};

		// Other
		visuals.dark_mode = match self {
			Theme::Dark => true,
			Theme::Light => false,
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => false,
		};
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
