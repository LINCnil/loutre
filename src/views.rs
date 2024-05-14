use crate::app::ChecksumApp;
use eframe::egui;

mod config;
mod main;

const UI_EXTRA_SPACE: f32 = 6.0;
const UI_BTN_PADDING_H: f32 = 10.0;
const UI_BTN_PADDING_V: f32 = 6.0;

#[derive(Clone)]
pub enum AppView {
	ConfigView,
	MainView,
}

impl Default for AppView {
	fn default() -> Self {
		Self::MainView
	}
}

impl AppView {
	pub fn display(&self, app: &mut ChecksumApp, ui: &mut egui::Ui) {
		match self {
			Self::ConfigView => config::display(app, ui),
			Self::MainView => main::display(app, ui),
		}
	}

	pub fn handle_dropped_files(&self, app: &mut ChecksumApp, ctx: &egui::Context) {
		match self {
			Self::ConfigView => {}
			Self::MainView => main::handle_dropped_files(app, ctx),
		}
	}
}
