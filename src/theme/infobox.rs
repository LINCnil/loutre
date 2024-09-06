use crate::theme::{Color, Icon, Theme};
use eframe::egui;

const INFOBOX_BORDER_SIZE: f32 = 1.0;
const INFOBOX_ICON_SIZE: f32 = 20.0;
const INFOBOX_MAIN_LEFT_BORDER_SIZE: f32 = 13.0;
const INFOBOX_PADDING: f32 = crate::UI_MARGIN_SMALL;
const INFOBOX_ROUNDING: f32 = crate::MAIN_ROUNDING;

#[derive(Clone, Copy, Debug)]
pub enum InfoBoxType {
	Full,
	Simple,
}

#[derive(Clone, Copy, Debug)]
pub enum InfoBoxLevel {
	Error,
	Info,
	Success,
	Warning,
}

impl InfoBoxLevel {
	fn get_icon(&self, theme: Theme) -> (Icon, egui::Color32) {
		match self {
			Self::Error => (Icon::SignError, Color::InfoBoxErrorIcon.get(theme)),
			Self::Info => (Icon::SignInfo, Color::InfoBoxInfoIcon.get(theme)),
			Self::Success => (Icon::SignSuccess, Color::InfoBoxSuccessIcon.get(theme)),
			Self::Warning => (Icon::SignWarning, Color::InfoBoxWarningIcon.get(theme)),
		}
	}

	fn render_icon(&self, theme: Theme) -> egui::RichText {
		let (icon, color) = self.get_icon(theme);
		egui::RichText::new(icon.get_char())
			.text_style(egui::TextStyle::Name("icon".into()))
			.size(INFOBOX_ICON_SIZE)
			.color(color)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct InfoBox {
	infobox_type: InfoBoxType,
	level: InfoBoxLevel,
	theme: Theme,
}

impl InfoBox {
	pub fn new(theme: Theme, infobox_type: InfoBoxType, level: InfoBoxLevel) -> Self {
		Self {
			infobox_type,
			level,
			theme,
		}
	}

	pub fn render_text(&self, ui: &mut egui::Ui, text: impl ToString) {
		let f = |ui: &mut egui::Ui| {
			ui.label(text.to_string());
		};
		match self.infobox_type {
			InfoBoxType::Full => self.render_full(ui, f),
			InfoBoxType::Simple => self.render_simple(ui, f),
		}
	}

	pub fn render_dyn<F>(&self, ui: &mut egui::Ui, function: F)
	where
		F: Fn(&mut egui::Ui),
	{
		match self.infobox_type {
			InfoBoxType::Full => self.render_full(ui, function),
			InfoBoxType::Simple => self.render_simple(ui, function),
		}
	}

	fn render_full<F>(&self, ui: &mut egui::Ui, function: F)
	where
		F: Fn(&mut egui::Ui),
	{
		ui.visuals_mut().override_text_color = Some(self.get_text_color());
		egui::Frame::none()
			.inner_margin(egui::Margin::from(0.0))
			.rounding(INFOBOX_ROUNDING)
			.fill(self.get_border_color())
			.stroke(egui::Stroke::new(
				INFOBOX_BORDER_SIZE,
				self.get_border_color(),
			))
			.show(ui, |ui| {
				egui::Frame::none()
					.outer_margin(egui::Margin {
						left: INFOBOX_MAIN_LEFT_BORDER_SIZE,
						right: INFOBOX_BORDER_SIZE,
						top: INFOBOX_BORDER_SIZE,
						bottom: INFOBOX_BORDER_SIZE,
					})
					.inner_margin(egui::Margin::from(INFOBOX_PADDING))
					.rounding(egui::Rounding {
						nw: 0.0,
						ne: INFOBOX_ROUNDING,
						sw: 0.0,
						se: INFOBOX_ROUNDING,
					})
					.fill(self.get_bg_color())
					.show(ui, |ui| {
						ui.horizontal_wrapped(|ui| {
							ui.label(self.level.render_icon(self.theme));
							function(ui);
							ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |_ui| {});
						});
					});
			});
		ui.visuals_mut().override_text_color = None;
	}

	fn render_simple<F>(&self, ui: &mut egui::Ui, function: F)
	where
		F: Fn(&mut egui::Ui),
	{
		ui.visuals_mut().override_text_color = Some(self.get_text_color());
		egui::Frame::none()
			.inner_margin(egui::Margin::from(INFOBOX_PADDING))
			.rounding(INFOBOX_ROUNDING)
			.fill(self.get_bg_color())
			.stroke(egui::Stroke::new(
				INFOBOX_BORDER_SIZE,
				self.get_border_color(),
			))
			.show(ui, |ui| {
				ui.horizontal_wrapped(|ui| {
					ui.label(self.level.render_icon(self.theme));
					function(ui);
					ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |_ui| {});
				});
			});
		ui.visuals_mut().override_text_color = None;
	}

	fn get_bg_color(&self) -> egui::Color32 {
		match self.level {
			InfoBoxLevel::Error => Color::InfoBoxErrorBackground.get(self.theme),
			InfoBoxLevel::Info => Color::InfoBoxInfoBackground.get(self.theme),
			InfoBoxLevel::Success => Color::InfoBoxSuccessBackground.get(self.theme),
			InfoBoxLevel::Warning => Color::InfoBoxWarningBackground.get(self.theme),
		}
	}

	fn get_border_color(&self) -> egui::Color32 {
		match self.level {
			InfoBoxLevel::Error => Color::InfoBoxErrorBorder.get(self.theme),
			InfoBoxLevel::Info => Color::InfoBoxInfoBorder.get(self.theme),
			InfoBoxLevel::Success => Color::InfoBoxSuccessBorder.get(self.theme),
			InfoBoxLevel::Warning => Color::InfoBoxWarningBorder.get(self.theme),
		}
	}

	fn get_text_color(&self) -> egui::Color32 {
		match self.level {
			InfoBoxLevel::Error => Color::InfoBoxErrorText.get(self.theme),
			InfoBoxLevel::Info => Color::InfoBoxInfoText.get(self.theme),
			InfoBoxLevel::Success => Color::InfoBoxSuccessText.get(self.theme),
			InfoBoxLevel::Warning => Color::InfoBoxWarningText.get(self.theme),
		}
	}
}
