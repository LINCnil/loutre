use crate::app::ChecksumApp;
use eframe::egui;

mod check_errors;
mod config;
mod main;

const UI_BTN_PADDING_H: f32 = 10.0;
const UI_BTN_PADDING_V: f32 = 6.0;
const UI_EXTRA_SPACE: f32 = 6.0;
const UI_LOGO_MARGIN: f32 = 16.0;

#[derive(Clone)]
pub enum AppView {
	CheckErrors,
	Config,
	Main,
}

impl Default for AppView {
	fn default() -> Self {
		Self::Main
	}
}

impl AppView {
	pub fn display(&self, app: &mut ChecksumApp, ui: &mut egui::Ui) {
		match self {
			Self::CheckErrors => check_errors::display(app, ui),
			Self::Config => config::display(app, ui),
			Self::Main => main::display(app, ui),
		}
	}

	pub fn handle_dropped_files(&self, app: &mut ChecksumApp, ctx: &egui::Context) {
		match self {
			Self::CheckErrors => {}
			Self::Config => {}
			Self::Main => main::handle_dropped_files(app, ctx),
		}
	}
}
