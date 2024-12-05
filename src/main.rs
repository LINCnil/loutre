mod analyse_hash;
mod app;
mod assets;
mod check;
mod clipboard;
mod components;
mod config;
mod content_file_format;
mod events;
mod files;
mod hash;
mod i18n;
mod nb_repr;
mod notifications;
mod parsers;
mod progress;
mod receipt;
mod serializers;
mod templates;
mod theme;
mod views;

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

const APP_NAME: &str = "LOUTRE — LOgiciel Unique de TRaitement des Empreintes";
const BUFF_SIZE: usize = 524_288; // 512 KiB
const BUFF_NOTIF_THRESHOLD: u64 = 700; // in milliseconds

#[cfg(unix)]
const CONFIG_FILE_DIR: &str = "cnil";
#[cfg(not(unix))]
const CONFIG_FILE_DIR: &str = "CNIL";
const CONFIG_FILE_SUBDIR: &str = "loutre";
const CONFIG_FILE_NAME: &str = "config.toml";

const DEFAULT_CLIPBOARD_THRESHOLD: usize = 42;
const PROGRESS_BAR_CHANNEL_CAPACITY: usize = 1024;

const WIN_WIDTH: u32 = 820;
const WIN_HEIGHT: u32 = 560;

fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	info!("starting app");

	// Config: https://github.com/DioxusLabs/dioxus/blob/main/packages/desktop/src/config.rs
	// WindowBuilder: https://docs.rs/tao/latest/tao/window/struct.WindowBuilder.html
	LaunchBuilder::desktop()
		.with_cfg(
			Config::new()
				.with_custom_head(get_custom_head())
				.with_menu(None)
				.with_window(
					WindowBuilder::new()
						.with_title(APP_NAME)
						.with_inner_size(LogicalSize::new(WIN_WIDTH, WIN_HEIGHT)),
				),
		)
		.launch(app::App)
}

fn get_custom_head() -> String {
	let mut ret = String::from("<style>");
	ret += include_str!("../assets/fonts/remixicon.css");
	ret += include_str!("../assets/loutre.css");
	ret += &get_css_font(assets::FONT_REMIXICON_B64, "remixicon", "woff2");
	ret += &get_css_font(assets::FONT_OPEN_SANS_B64, "Open Sans", "woff2");
	ret += "</style>";
	ret
}

fn get_css_font(b64: &str, family: &str, format: &str) -> String {
	format!(
		"\n@font-face {{ font-family: '{}'; src: url({}) format('{}'); }}",
		family, b64, format
	)
}
