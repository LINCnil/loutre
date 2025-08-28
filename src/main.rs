#![cfg_attr(windows, windows_subsystem = "windows")]

mod analyse_hash;
mod app;
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

use dioxus::desktop::tao::window::Icon;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

const APP_NAME: &str = "LOUTRE — LOgiciel Unique de TRaitement des Empreintes";
const BUFF_SIZE: usize = 524_288; // 512 KiB
const BUFF_NOTIF_THRESHOLD: u64 = 700; // in milliseconds

#[cfg(unix)]
const CONFIG_FILE_DIR: &str = "cnil";
#[cfg(not(unix))]
const CONFIG_FILE_DIR: &str = "CNIL";
const CONFIG_FILE_SUBDIR: &str = "loutre";
#[cfg(not(feature = "nightly"))]
const CONFIG_FILE_NAME: &str = "config.toml";
#[cfg(feature = "nightly")]
const CONFIG_FILE_NAME: &str = "config.nightly.toml";

const DEFAULT_CLIPBOARD_THRESHOLD: usize = 42;

const WIN_WIDTH: u32 = 820;
const WIN_HEIGHT: u32 = 560;

fn main() {
	tracing::info!("starting app");

	let raw_ico = include_bytes!("../assets/icon_rgba8.bin").to_vec();
	let ico = Icon::from_rgba(raw_ico, 460, 460).unwrap();

	let has_decorations_env_str =
		std::env::var("LOUTRE_WITH_DECORATION").unwrap_or("true".to_string());
	let has_decorations = str_is_trueish(&has_decorations_env_str);

	// Config: https://docs.rs/dioxus-desktop/latest/dioxus_desktop/struct.Config.html
	// WindowBuilder: https://docs.rs/tao/latest/tao/window/struct.WindowBuilder.html
	LaunchBuilder::desktop()
		.with_cfg(
			Config::new()
				.with_menu(None)
				.with_window(
					WindowBuilder::new()
						.with_title(APP_NAME)
						.with_decorations(has_decorations)
						.with_inner_size(LogicalSize::new(WIN_WIDTH, WIN_HEIGHT)),
				)
				.with_icon(ico),
		)
		.launch(app::App)
}

fn str_is_trueish(s: &str) -> bool {
	let trueish_lst = ["1", "t", "true", "y", "yes"];
	trueish_lst.contains(&s.to_lowercase().as_str())
}
