use crate::i18n::I18n;
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, TextStyle};
use serde::{Deserialize, Serialize};

pub const AVAILABLE_THEMES: &[Theme] = &[Theme::Dark, Theme::Light];
const LABEL_BORDER_SIZE: f32 = 1.0;
const LABEL_ICON_SIZE: f32 = 20.0;
const LABEL_MAIN_LEFT_BORDER_SIZE: f32 = 13.0;
const LABEL_PADDING: f32 = 6.0;
const LABEL_ROUNDING: f32 = 7.0;
const SIGN_INFO: char = '\u{F449}';
const SIGN_SUCCESS: char = '\u{EB81}';
const SIGN_WARNING: char = '\u{EA21}';

struct LabelColors {
	background: Color32,
	border: Color32,
	icon: Color32,
	font: Color32,
}

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
			Theme::Dark => include_bytes!("../assets/main-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/main-logo-light.png").to_vec(),
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => {
				include_bytes!("../assets/main-logo-nightly.png").to_vec()
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
		colors: LabelColors,
		is_main: bool,
		extra: F,
	) where
		F: Fn(&mut egui::Ui),
	{
		if is_main {
			egui::Frame::none()
				.inner_margin(egui::Margin::from(0.0))
				.rounding(LABEL_ROUNDING)
				.fill(colors.border)
				.stroke(egui::Stroke::new(LABEL_BORDER_SIZE, colors.border))
				.show(ui, |ui| {
					egui::Frame::none()
						.outer_margin(egui::Margin {
							left: LABEL_MAIN_LEFT_BORDER_SIZE,
							right: LABEL_BORDER_SIZE,
							top: LABEL_BORDER_SIZE,
							bottom: LABEL_BORDER_SIZE,
						})
						.inner_margin(egui::Margin::from(LABEL_PADDING))
						.rounding(egui::Rounding {
							nw: 0.0,
							ne: LABEL_ROUNDING,
							sw: 0.0,
							se: LABEL_ROUNDING,
						})
						.fill(colors.background)
						.show(ui, |ui| {
							ui.horizontal(|ui| {
								ui.label(icon.size(LABEL_ICON_SIZE).color(colors.icon));
								ui.add(egui::Label::new(RichText::new(text).color(colors.font)).wrap(true));
								extra(ui);
								ui.with_layout(
									egui::Layout::right_to_left(egui::Align::TOP),
									|_ui| {},
								);
							});
						});
				});
		} else {
			egui::Frame::none()
				.inner_margin(egui::Margin::from(LABEL_PADDING))
				.fill(colors.background)
				.rounding(LABEL_ROUNDING)
				.stroke(egui::Stroke::new(LABEL_BORDER_SIZE, colors.border))
				.show(ui, |ui| {
					ui.horizontal(|ui| {
						ui.label(icon.size(LABEL_ICON_SIZE).color(colors.icon));
						ui.add(egui::Label::new(RichText::new(text).color(colors.font)).wrap(true));
						extra(ui);
						ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |_ui| {});
					});
				});
		};
	}

	pub fn add_error_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_label(
			ui,
			text,
			self.icon(SIGN_SUCCESS),
			self.get_error_label_colors(),
			false,
			|_| {},
		);
	}

	pub fn add_info_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_info_label_extra(ui, text, |_| {});
	}

	pub fn add_info_label_extra<F>(&self, ui: &mut egui::Ui, text: &str, extra: F)
	where
		F: Fn(&mut egui::Ui),
	{
		self.add_label(
			ui,
			text,
			self.icon(SIGN_INFO),
			self.get_info_label_colors(),
			true,
			extra,
		);
	}

	pub fn add_success_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_label(
			ui,
			text,
			self.icon(SIGN_SUCCESS),
			self.get_success_label_colors(),
			false,
			|_| {},
		);
	}

	pub fn add_warning_label(&self, ui: &mut egui::Ui, text: &str) {
		self.add_label(
			ui,
			text,
			self.icon(SIGN_WARNING),
			self.get_warning_label_colors(),
			true,
			|_| {},
		);
	}

	fn get_error_label_colors(&self) -> LabelColors {
		match self {
			Theme::Dark => LabelColors {
				background: Color32::from_rgb(0xff, 0xf0, 0xf0),
				border: Color32::from_rgb(0xac, 0x21, 0x21),
				icon: Color32::from_rgb(0x21, 0xac, 0x59),
				font: Color32::from_rgb(0x21, 0xac, 0x59),
			},
			Theme::Light => LabelColors {
				background: Color32::from_rgb(0xff, 0xf0, 0xf0),
				border: Color32::from_rgb(0xac, 0x21, 0x21),
				icon: Color32::from_rgb(0x21, 0xac, 0x59),
				font: Color32::from_rgb(0x21, 0xac, 0x59),
			},
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => LabelColors {
				background: Color32::from_rgb(0xff, 0xf0, 0xf0),
				border: Color32::from_rgb(0xac, 0x21, 0x21),
				icon: Color32::from_rgb(0x21, 0xac, 0x59),
				font: Color32::from_rgb(0x21, 0xac, 0x59),
			},
		}
	}

	fn get_info_label_colors(&self) -> LabelColors {
		match self {
			Theme::Dark => LabelColors {
				background: Color32::from_rgb(0x34, 0x8c, 0xff),
				border: Color32::from_rgb(0x34, 0x8c, 0xff),
				icon: Color32::from_rgb(0x34, 0x8c, 0xff),
				font: Color32::from_rgb(0xe6, 0xf1, 0xff),
			},
			Theme::Light => LabelColors {
				background: Color32::from_rgb(0xbb, 0xe4, 0xff),
				border: Color32::from_rgb(0x34, 0x8c, 0xff),
				icon: Color32::from_rgb(0x34, 0x8c, 0xff),
				font: Color32::from_rgb(0x41, 0x41, 0x41),
			},
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => LabelColors {
				background: Color32::from_rgb(0xbb, 0xe4, 0xff),
				border: Color32::from_rgb(0x34, 0x8c, 0xff),
				icon: Color32::from_rgb(0x34, 0x8c, 0xff),
				font: Color32::from_rgb(0x41, 0x41, 0x41),
			},
		}
	}

	fn get_success_label_colors(&self) -> LabelColors {
		match self {
			Theme::Dark => LabelColors {
				background: Color32::from_rgb(0x10, 0x64, 0x32),
				border: Color32::from_rgb(0x34, 0xff, 0x86),
				icon: Color32::from_rgb(0x34, 0xff, 0x86),
				font: Color32::from_rgb(0xde, 0xff, 0xeb),
			},
			Theme::Light => LabelColors {
				background: Color32::from_rgb(0xe5, 0xff, 0xf0),
				border: Color32::from_rgb(0x21, 0xac, 0x59),
				icon: Color32::from_rgb(0x21, 0xac, 0x59),
				font: Color32::from_rgb(0x21, 0xac, 0x59),
			},
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => LabelColors {
				background: Color32::from_rgb(0xe5, 0xff, 0xf0),
				border: Color32::from_rgb(0x21, 0xac, 0x59),
				icon: Color32::from_rgb(0x21, 0xac, 0x59),
				font: Color32::from_rgb(0x21, 0xac, 0x59),
			},
		}
	}

	fn get_warning_label_colors(&self) -> LabelColors {
		match self {
			Theme::Dark => LabelColors {
				background: Color32::from_rgb(0x9b, 0x7b, 0x23),
				border: Color32::from_rgb(0xff, 0xd1, 0x51),
				icon: Color32::from_rgb(0xff, 0xd1, 0x51),
				font: Color32::from_rgb(0xff, 0xf8, 0x6e),
			},
			Theme::Light => LabelColors {
				background: Color32::from_rgb(0xff, 0xf8, 0xe5),
				border: Color32::from_rgb(0xff, 0xd1, 0x51),
				icon: Color32::from_rgb(0xff, 0xd1, 0x51),
				font: Color32::from_rgb(0x41, 0x41, 0x41),
			},
			#[cfg(feature = "nightly")]
			Theme::NightlyDark | Theme::NightlyLight => LabelColors {
				background: Color32::from_rgb(0xff, 0xf8, 0xe5),
				border: Color32::from_rgb(0xff, 0xd1, 0x51),
				icon: Color32::from_rgb(0xff, 0xd1, 0x51),
				font: Color32::from_rgb(0x41, 0x41, 0x41),
			},
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
