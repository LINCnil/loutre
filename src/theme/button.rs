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
		let mut job = egui::text::LayoutJob::default();
		if let Some(icon) = &self.icon {
			job.append(
				icon.get_char().to_string().as_str(),
				0.0,
				egui::TextFormat {
					font_id: egui::FontId::new(ICON_SIZE, egui::FontFamily::Name("icon".into())),
					..Default::default()
				},
			);
		}
		if let Some(text) = &self.text {
			job.append(
				&format!(" {text}"),
				0.0,
				egui::TextFormat {
					font_id: egui::FontId::new(TEXT_SIZE, egui::FontFamily::Proportional),
					..Default::default()
				},
			);
		}
		egui::Button::new(job)
	}
}
