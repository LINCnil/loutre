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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
	Dark,
	#[default]
	Light,
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
		fonts
			.families
			.entry(FontFamily::Proportional)
			.or_default()
			.push("RemixIcon".to_owned());

		ctx.set_fonts(fonts);

		let mut style = (*ctx.style()).clone();
		style.text_styles = [
			(
				TextStyle::Heading,
				FontId::new(20.0, FontFamily::Name("bold".into())),
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

	pub fn title(&self, text: &str) -> RichText {
		RichText::new(text)
			.text_style(TextStyle::Heading)
			.color(Color::Title.get(*self))
	}

	pub fn icon(&self, icon: char) -> RichText {
		RichText::new(icon).text_style(TextStyle::Name("icon".into()))
	}

	pub fn icon_with_txt(&self, icon: char, text: &str) -> RichText {
		RichText::new(format!("{icon} {text}")).text_style(TextStyle::Name("icon".into()))
	}

	pub fn get_icon_bytes(&self) -> Vec<u8> {
		if cfg!(feature = "nightly") {
			include_bytes!("../assets/ico_nightly/32-32.png").to_vec()
		} else {
			include_bytes!("../assets/ico/32-32.png").to_vec()
		}
	}

	pub fn get_logo_bytes(&self) -> (String, Vec<u8>) {
		if cfg!(feature = "nightly") {
			(
				"bytes://logo-nightly".to_string(),
				include_bytes!("../assets/main-logo-nightly.png").to_vec(),
			)
		} else {
			match self {
				Theme::Dark => (
					"bytes://logo-dark".to_string(),
					include_bytes!("../assets/main-logo-dark.png").to_vec(),
				),
				Theme::Light => (
					"bytes://logo-light".to_string(),
					include_bytes!("../assets/main-logo-light.png").to_vec(),
				),
			}
		}
	}

	pub fn display(&self, i18n: &I18n) -> String {
		match self {
			Theme::Dark => i18n.msg("theme_dark"),
			Theme::Light => i18n.msg("theme_light"),
		}
	}

	pub fn get_main_frame(&self) -> egui::Frame {
		egui::Frame::default()
			.inner_margin(crate::UI_MARGIN_NONE)
			.fill(Color::MainFrameBackground.get(*self))
	}

	pub fn set_visuals(&self, visuals: &mut egui::style::Visuals) {
		// See also:
		// https://docs.rs/egui/latest/egui/style/struct.Visuals.html
		// https://docs.rs/egui/latest/egui/style/struct.Widgets.html
		// https://docs.rs/egui/latest/egui/style/struct.WidgetVisuals.html

		visuals.selection = egui::style::Selection {
			bg_fill: Color::ButtonBackground.get(*self),
			stroke: egui::Stroke {
				width: 0.0,
				color: Color::MainText.get(*self),
			},
		};
		visuals.extreme_bg_color = Color::ButtonBackground.get(*self);

		// Widgets (non interactive)
		visuals.widgets.noninteractive.bg_fill = Color::ButtonBackground.get(*self);
		visuals.widgets.noninteractive.weak_bg_fill = visuals.widgets.noninteractive.bg_fill;
		visuals.widgets.noninteractive.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::MainText.get(*self),
		};
		visuals.widgets.noninteractive.rounding = crate::MAIN_ROUNDING.into();
		visuals.widgets.noninteractive.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::MainText.get(*self),
		};

		// Widgets (interactive - default)
		visuals.widgets.inactive.bg_fill = Color::ButtonBackground.get(*self);
		visuals.widgets.inactive.weak_bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.inactive.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorder.get(*self),
		};
		visuals.widgets.inactive.rounding = crate::MAIN_ROUNDING.into();
		visuals.widgets.inactive.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonText.get(*self),
		};

		// Widgets (interactive - hovered)
		visuals.widgets.hovered.bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.hovered.weak_bg_fill = Color::ButtonBackgroundHovered.get(*self);
		visuals.widgets.hovered.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorderHovered.get(*self),
		};
		visuals.widgets.hovered.rounding = crate::MAIN_ROUNDING.into();
		visuals.widgets.hovered.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonTextHovered.get(*self),
		};

		// Widgets (interactive - active)
		visuals.widgets.active.bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.active.weak_bg_fill = Color::ButtonBackgroundHovered.get(*self);
		visuals.widgets.active.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorderHovered.get(*self),
		};
		visuals.widgets.active.rounding = crate::MAIN_ROUNDING.into();
		visuals.widgets.active.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonTextHovered.get(*self),
		};

		// Widgets (interactive - active)
		visuals.widgets.open.bg_fill = visuals.widgets.inactive.bg_fill;
		visuals.widgets.open.weak_bg_fill = Color::ButtonBackgroundHovered.get(*self);
		visuals.widgets.open.bg_stroke = egui::Stroke {
			width: 1.0,
			color: Color::ButtonBorderHovered.get(*self),
		};
		visuals.widgets.open.rounding = crate::MAIN_ROUNDING.into();
		visuals.widgets.open.fg_stroke = egui::Stroke {
			width: 12.0,
			color: Color::ButtonTextHovered.get(*self),
		};

		// Other
		visuals.dark_mode = match self {
			Theme::Dark => true,
			Theme::Light => false,
		};
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
