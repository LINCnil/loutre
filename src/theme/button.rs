use crate::theme::{Color, Icon, Theme};
use eframe::egui;

const BUTTON_BORDER_SIZE: f32 = 1.0;

#[derive(Clone, Copy, Debug)]
pub enum ButtonStyle {
	MainDark,
	MainLight,
}

impl ButtonStyle {
	fn color_background(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::MainDark => Color::ButtonMainDarkBackground,
			Self::MainLight => Color::ButtonMainLightBackground,
		}
		.get(theme)
	}

	fn color_border(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::MainDark => Color::ButtonMainDarkBorder,
			Self::MainLight => Color::ButtonMainLightBorder,
		}
		.get(theme)
	}

	fn color_icon(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::MainDark => Color::ButtonMainDarkIcon,
			Self::MainLight => Color::ButtonMainLightIcon,
		}
		.get(theme)
	}

	fn color_text(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::MainDark => Color::ButtonMainDarkText,
			Self::MainLight => Color::ButtonMainLightText,
		}
		.get(theme)
	}

	fn size_icon(&self) -> f32 {
		match self {
			Self::MainDark => 20.0,
			Self::MainLight => 20.0,
		}
	}

	fn size_text(&self) -> f32 {
		match self {
			Self::MainDark => 12.0,
			Self::MainLight => 12.0,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Button {
	theme: Theme,
	style: ButtonStyle,
	icon: Option<Icon>,
	text: Option<String>,
}

impl Button {
	pub fn new(theme: Theme, style: ButtonStyle) -> Self {
		Self {
			theme,
			style,
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
					font_id: egui::FontId::new(
						self.style.size_icon(),
						egui::FontFamily::Name("icon".into()),
					),
					color: self.style.color_icon(self.theme),
					..Default::default()
				},
			);
		}
		if let Some(text) = &self.text {
			job.append(
				&format!(" {text}"),
				0.0,
				egui::TextFormat {
					font_id: egui::FontId::new(
						self.style.size_text(),
						egui::FontFamily::Proportional,
					),
					color: self.style.color_text(self.theme),
					..Default::default()
				},
			);
		}
		egui::Button::new(job)
			.stroke(egui::Stroke {
				width: BUTTON_BORDER_SIZE,
				color: self.style.color_border(self.theme),
			})
			.fill(self.style.color_background(self.theme))
	}
}
