#![allow(non_snake_case)]

use crate::components::{ConfigMenu, ConfigMenuHighlight, DropZone, Grid, Header};
use crate::config::Config;
use crate::parsers::parse_bool;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn MainConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();

	rsx! {
		DropZone {
			Header {
				is_config_view: true,
				h1 {
					{ t!("view_config_title") }
				}
			}
			ConfigMenu { hl: ConfigMenuHighlight::Main }
			Grid {
				p {
					label {
						r#for: "cfg_main_empty_files_warning",
						{ t!("view_config_main_msg_empty_files_warning") }
					}
				}
				div {
					input {
						id: "cfg_main_empty_files_warning",
						r#type: "checkbox",
						checked: cfg_sig().is_empty_file_warning_enabled(),
						onchange: move |event| {
							let new_value = parse_bool(&event.data.value());
							spawn(async move {
								let mut cfg = cfg_sig();
								cfg.enable_empty_file_warning = Some(new_value);
								cfg.write_to_file();
								cfg_sig.set(cfg);
							});
						},
					}
				}
			}
		}
	}
}
