#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::config::{ConfigElement, ConfigMenu, ConfigMenuHighlight};
use crate::components::{Checkbox, DropZone, Header, MainSection};
use crate::config::Config;
use crate::parsers::parse_bool;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn MessagesConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();

	rsx! {
		DropZone {
			Header {
				is_config_view: true,
			}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ t!("view_config_title") }
				}
				ConfigMenu { hl: ConfigMenuHighlight::Messages }
				ConfigElement {
					// Empty files warning
					p {
						label {
							r#for: "cfg_main_empty_files_warning",
							{ t!("view_config_messages_msg_empty_files_warning") }
						}
					}
					div {
						Checkbox {
							id: "cfg_main_empty_files_warning",
							name: "cfg_main_empty_files_warning",
							checked: cfg_sig().is_empty_file_warning_enabled(),
							onchange: move |event: FormEvent| {
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
				ConfigElement {
					// Duplicated files warning
					p {
						label {
							r#for: "cfg_main_duplicated_files_warning",
							{ t!("view_config_messages_msg_duplicated_files_warning") }
						}
					}
					div {
						Checkbox {
							id: "cfg_main_duplicated_files_warning",
							name: "cfg_main_duplicated_files_warning",
							checked: cfg_sig().is_duplicate_file_warning_enabled(),
							onchange: move |event: FormEvent| {
								let new_value = parse_bool(&event.data.value());
								spawn(async move {
									let mut cfg = cfg_sig();
									cfg.enable_duplicate_file_warning = Some(new_value);
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
}
