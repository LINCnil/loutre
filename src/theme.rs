use crate::i18n::I18n;
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, TextStyle};
use serde::{Deserialize, Serialize};

pub const AVAILABLE_THEMES: &[Theme] = &[Theme::Dark, Theme::Light];
const SIGN_INFO: char = '\u{F449}';
const SIGN_SUCCESS: char = '\u{EB81}';
const SIGN_WARNING: char = '\u{EA21}';

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

	fn add_label<F>(
		&self,
		ui: &mut egui::Ui,
		text: &str,
		icon: RichText,
		bg_color: Color32,
		border_color: Color32,
		extra: F,
	) where
		F: Fn(&mut egui::Ui),
	{
		let margin = egui::Margin::from(6.0);
		egui::Frame::none()
			.inner_margin(margin)
			.fill(bg_color)
			.rounding(7.0f32)
			.stroke(egui::Stroke::new(1.0, border_color))
			.show(ui, |ui| {
				ui.horizontal(|ui| {
					ui.label(icon.size(20.0));
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
		let (bg, border) = self.get_info_label_colors();
		self.add_label(ui, text, self.icon(SIGN_INFO), bg, border, extra);
	}

	pub fn add_success_label(&self, ui: &mut egui::Ui, text: &str) {
		let (bg, border) = self.get_success_label_colors();
		self.add_label(ui, text, self.icon(SIGN_SUCCESS), bg, border, |_| {});
	}

	pub fn add_warning_label(&self, ui: &mut egui::Ui, text: &str) {
		let (bg, border) = self.get_warning_label_colors();
		self.add_label(ui, text, self.icon(SIGN_WARNING), bg, border, |_| {});
	}

	fn get_info_label_colors(&self) -> (Color32, Color32) {
		match self {
			Theme::Dark => (
				Color32::from_rgb(0x34, 0x8c, 0xff),
				Color32::from_rgb(0x34, 0x8c, 0xff),
			),
			Theme::Light => (
				Color32::from_rgb(0xbb, 0xe4, 0xff),
				Color32::from_rgb(0x34, 0x8c, 0xff),
			),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => (
				Color32::from_rgb(0xbb, 0xe4, 0xff),
				Color32::from_rgb(0x34, 0x8c, 0xff),
			),
		}
	}

	fn get_success_label_colors(&self) -> (Color32, Color32) {
		match self {
			Theme::Dark => (
				Color32::from_rgb(0x10, 0x64, 0x32),
				Color32::from_rgb(0x34, 0xff, 0x86),
			),
			Theme::Light => (
				Color32::from_rgb(0xe5, 0xff, 0xf0),
				Color32::from_rgb(0x21, 0xac, 0x59),
			),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => (
				Color32::from_rgb(0xe5, 0xff, 0xf0),
				Color32::from_rgb(0x21, 0xac, 0x59),
			),
		}
	}

	fn get_warning_label_colors(&self) -> (Color32, Color32) {
		match self {
			Theme::Dark => (
				Color32::from_rgb(0x9b, 0x7b, 0x23),
				Color32::from_rgb(0xff, 0xd1, 0x51),
			),
			Theme::Light => (
				Color32::from_rgb(0xff, 0xf8, 0xe5),
				Color32::from_rgb(0xff, 0xd1, 0x51),
			),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => (
				Color32::from_rgb(0xff, 0xf8, 0xe5),
				Color32::from_rgb(0xff, 0xd1, 0x51),
			),
		}
	}

	pub fn get_main_frame(&self) -> egui::Frame {
		egui::Frame::default().inner_margin(8.0).fill(match self {
			Theme::Dark => Color32::from_rgb(0x17, 0x17, 0x2f),
			Theme::Light => Color32::from_rgb(0xf8, 0xf8, 0xf8),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => Color32::from_rgb(0xf8, 0xf8, 0xf8),
		})
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
