#![allow(non_snake_case)]

use crate::clipboard::ClipboardPersistence;
use crate::components::{
	ConfigMenu, ConfigMenuHighlight, DropZone, Grid, Header, Select, SelectOption,
};
use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::str::FromStr;

#[component]
pub fn ClipboardConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
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

	rsx! {
		DropZone {
			Header {
				is_config_view: true,
				h1 {
					{ t!("view_config_title") }
				}
			}
			ConfigMenu { hl: ConfigMenuHighlight::Clipboard }
			form {
				Grid {
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
							value: cfg_sig().get_clipboard_threshold().to_string(),
							r#type: "number",
							min: 1,
							onchange: move |event: FormEvent| {
								if let Ok(nb) = event.data.value().as_str().parse() {
									spawn(async move {
										let mut cfg = cfg_sig();
										cfg.clipboard_threshold = Some(nb);
										cfg.write_to_file();
										cfg_sig.set(cfg);
									});
								}
							}
						}
					}

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
							selected_option: cfg_sig().hash_function.to_string().to_lowercase(),
							onchange: move |event: FormEvent| {
								if let Ok(new_value) = ClipboardPersistence::from_str(&event.data.value()) {
									spawn(async move {
										let mut cfg = cfg_sig();
										cfg.clipboard_persistence = new_value.into();
										cfg.write_to_file();
										cfg_sig.set(cfg);
									});
								}
							},
						}
					}
				}
			}
		}
	}
}
