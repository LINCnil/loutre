use crate::theme::Icon;
use eframe::egui;

const ICON_SIZE: f32 = 20.0;
const TEXT_SIZE: f32 = 12.0;

#[derive(Clone, Debug)]
pub struct Button {
	icon: Option<Icon>,
	text: Option<String>,
}

impl Button {
	pub fn new() -> Self {
		Self {
			icon: None,
			text: None,
		}
	}

	pub fn icon(mut self, icon: Icon) -> Self {
		self.icon = Some(icon);
		self
	}

	pub fn text(mut self, text: impl ToString) -> Self {
		self.text = Some(text.to_string());
		self
	}

	pub fn render(&self) -> egui::Button {
		if let Some(text) = &self.text {
			return egui::Button::new(text);
		}
		if let Some(icon) = &self.icon {
			let icon = egui::RichText::new(icon.get_char().to_string())
				.family(egui::FontFamily::Name("icon".into()))
				.size(ICON_SIZE);
			return egui::Button::new(icon);
		}
		unreachable!();
	}
}
