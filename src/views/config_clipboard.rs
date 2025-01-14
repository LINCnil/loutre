#![allow(non_snake_case)]

use crate::app::Route;
use crate::clipboard::{ClipboardPersistence, ClipboardStart};
use crate::components::config::{ConfigElement, ConfigMenu, ConfigMenuHighlight};
use crate::components::{ApplyConfig, Header, MainSection, Root, Select, SelectOption};
use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::str::FromStr;

#[component]
pub fn ClipboardConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut clipboard_start_sig = use_context::<Signal<ClipboardStart>>();
	let cl_pers_opts = vec![
		SelectOption::new(
			t!("view_config_clipboard_msg_persistence_default"),
			ClipboardPersistence::Default.to_string(),
		),
		SelectOption::new(
			t!("view_config_clipboard_msg_persistence_activated"),
			ClipboardPersistence::Activated.to_string(),
		),
		SelectOption::new(
			t!("view_config_clipboard_msg_persistence_deactivated"),
			ClipboardPersistence::Deactivated.to_string(),
		),
	];
	let mut clipboard_start = use_signal(&*clipboard_start_sig);
	let mut clipboard_threshold = use_signal(|| cfg_sig().get_clipboard_threshold());
	let mut clipboard_persistence = use_signal(|| {
		let cp: ClipboardPersistence = cfg_sig().clipboard_persistence.into();
		cp
	});

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ t!("view_config_title") }
				}
				ConfigMenu { hl: ConfigMenuHighlight::Clipboard }
				form {
					ConfigElement {
						// First evidence number
						p {
							label {
								r#for: "cfg_clipboard_clipboard_start",
								{ t!("view_config_clipboard_start_msg") }
							}
						}
						div {
							input {
								id: "cfg_clipboard_clipboard_start",
								name: "cfg_clipboard_clipboard_start",
								value: clipboard_start().to_string(),
								r#type: "number",
								min: 1,
								onchange: move |event: FormEvent| {
									if let Ok(nb) = event.data.value().as_str().parse::<usize>() {
										clipboard_start.set(nb.into());
									}
								}
							}
						}
					}
					ConfigElement {
						// Cliboard threshold
						p {
							label {
								r#for: "cfg_clipboard_threshold",
								{ t!("view_config_clipboard_msg_threshold") }
							}
						}
						div {
							input {
								id: "cfg_clipboard_threshold",
								name: "cfg_clipboard_threshold",
								value: clipboard_threshold().to_string(),
								r#type: "number",
								min: 1,
								onchange: move |event: FormEvent| {
									if let Ok(nb) = event.data.value().as_str().parse() {
										clipboard_threshold.set(nb);
									}
								}
							}
						}
					}
					ConfigElement {
						// Cliboard persistence
						p {
							label {
								r#for: "cfg_clipboard_persistence",
								{ t!("view_config_clipboard_msg_persistence") }
							}
						}
						div {
							Select {
								id: "cfg_clipboard_persistence",
								name: "cfg_clipboard_persistence",
								options: cl_pers_opts,
								selected_option: clipboard_persistence().to_string(),
								onchange: move |event: FormEvent| {
									if let Ok(new_value) = ClipboardPersistence::from_str(&event.data.value()) {
										clipboard_persistence.set(new_value);
									}
								},
							}
						}
					}
				}
				ApplyConfig {
					onclick: move |_event| {
						let new_clipboard_start = clipboard_start();
						let new_clipboard_threshold = clipboard_threshold();
						let new_clipboard_persistence = clipboard_persistence();
						spawn(async move {
							clipboard_start_sig.set(new_clipboard_start);
							let mut cfg = cfg_sig();
							cfg.clipboard_threshold = Some(new_clipboard_threshold);
							cfg.clipboard_persistence = new_clipboard_persistence.into();
							cfg.write_to_file();
							cfg_sig.set(cfg);
						});
					},
				}
			}
		}
	}
}
