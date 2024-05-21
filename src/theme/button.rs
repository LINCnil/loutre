use crate::theme::Icon;
use eframe::egui;

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
		let mut txt = Vec::with_capacity(2);
		if let Some(icon) = &self.icon {
			txt.push(icon.get_char().to_string());
		}
		if let Some(text) = &self.text {
			txt.push(text.to_string());
		}
		let txt = txt.join(" ");
		egui::Button::new(txt)
	}
}
