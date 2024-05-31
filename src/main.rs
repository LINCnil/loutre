#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod checker;
mod clipboard;
mod config;
mod content_file;
mod file;
mod file_list;
mod hasher;
mod i18n;
mod nb_repr;
mod path_cmp;
mod receipt;
mod theme;
mod views;

const APP_NAME: &str = "LOUTRE — LOgiciel Unique de TRaitement des Empreintes";
const BUFF_SIZE: usize = 524288; // 512 KiB
const BUFF_NOTIF_THRESHOLD: u64 = 700; // in milliseconds
const DEFAULT_LANG: &str = "fr-FR";

#[cfg(unix)]
const CONFIG_FILE_DIR: &str = "cnil";
#[cfg(not(unix))]
const CONFIG_FILE_DIR: &str = "CNIL";
const CONFIG_FILE_SUBDIR: &str = "loutre";
const CONFIG_FILE_NAME: &str = "config.toml";

#[cfg(windows)]
const CONTENT_FILE_PATH_PREFIX: &str = "\\";
#[cfg(not(windows))]
const CONTENT_FILE_PATH_PREFIX: &str = "";

const NB_FILES_START: u32 = 1;
const WIN_WIDTH: f32 = 720.0;
const WIN_HEIGHT: f32 = 345.0;

fn main() {
	let config = config::Config::init();
	let viewport = eframe::egui::ViewportBuilder::default()
		.with_decorations(true)
		.with_drag_and_drop(true)
		.with_inner_size(eframe::egui::Vec2 {
			x: WIN_WIDTH,
			y: WIN_HEIGHT,
		});
	let viewport = match get_app_icon(&config.theme) {
		Some(icon_data) => viewport.with_icon(icon_data),
		None => viewport,
	};
	let win_opts = eframe::NativeOptions {
		viewport,
		default_theme: config.theme.into(),
		..Default::default()
	};
	let app = app::ChecksumApp::new(&config);
	let app_name = format!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION"));
	if let Err(e) = eframe::run_native(
		&app_name,
		win_opts,
		Box::new(|cc| Box::new(app.init_theme(cc))),
	) {
		eprintln!("Error: {e}");
	}
}

fn get_app_icon(theme: &theme::Theme) -> Option<eframe::egui::IconData> {
	let icon_res =
		image::load_from_memory_with_format(&theme.get_icon_bytes(), image::ImageFormat::Png);
	let icon = match icon_res {
		Ok(img) => img.into_rgba8(),
		Err(_) => {
			return None;
		}
	};
	let (width, height) = icon.dimensions();
	Some(eframe::egui::IconData {
		rgba: icon.into_raw(),
		width,
		height,
	})
}
