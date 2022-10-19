#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod checker;
mod clipboard;
mod email;
mod file;
mod file_list;
mod hasher;
mod path_cmp;
mod theme;

const APP_NAME: &str = "LOUTRE — LOgiciel Unique de TRaitement des Empreintes";
const BUFF_SIZE: usize = 32768;
const BUFF_NOTIF_THRESHOLD: u64 = 700;
const CONTENT_FILE_HEADER: &str = "Nom du document\tTaille (octets)\t SHA256\r\n";
const CONTENT_FILE_NAME: &str = "contenu.txt";
#[cfg(windows)]
const CONTENT_FILE_PATH_PREFIX: &str = "\\";
#[cfg(not(windows))]
const CONTENT_FILE_PATH_PREFIX: &str = "";

const NB_FILES_START: u32 = 1;
const NB_FILES_WARN_THRESHOLD: usize = 30;
const WIN_WIDTH: f32 = 720.0;
const WIN_HEIGHT: f32 = 310.0;

fn main() {
	let theme = theme::Theme::default();
	let win_opts = eframe::NativeOptions {
		decorated: true,
		drag_and_drop_support: true,
		icon_data: get_app_icon(&theme),
		initial_window_size: Some(eframe::egui::Vec2 {
			x: WIN_WIDTH,
			y: WIN_HEIGHT,
		}),
		default_theme: theme.clone().into(),
		..Default::default()
	};
	let app = app::ChecksumApp::new(&theme);
	eframe::run_native(APP_NAME, win_opts, Box::new(|_cc| Box::new(app)));
}

fn get_app_icon(theme: &theme::Theme) -> Option<eframe::IconData> {
	let icon_res =
		image::load_from_memory_with_format(&theme.get_icon_bytes(), image::ImageFormat::Png);
	let icon = match icon_res {
		Ok(img) => img.into_rgba8(),
		Err(_) => {
			return None;
		}
	};
	let (width, height) = icon.dimensions();
	Some(eframe::IconData {
		rgba: icon.into_raw(),
		width,
		height,
	})
}
