#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::config::{ConfigElement, ConfigMenu, ConfigMenuHighlight};
use crate::components::{ApplyConfig, Checkbox, Header, MainSection, Root};
use crate::config::Config;
use crate::parsers::parse_bool;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn FilesConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut include_hidden_files = use_signal(|| cfg_sig().include_hidden_files());
	let mut include_system_files = use_signal(|| cfg_sig().include_system_files());
	let mut set_files_readonly = use_signal(|| cfg_sig().set_files_as_readonly());

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ tid!("view_config_title") }
				}
				ConfigMenu { hl: ConfigMenuHighlight::Files }
				form {
					// Include hidden files
					ConfigElement {
						id: "cfg_main_include_hidden_files",
						label: tid!("view_config_main_msg_include_hidden_files"),
						Checkbox {
							id: "cfg_main_include_hidden_files",
							name: "cfg_main_include_hidden_files",
							checked: include_hidden_files(),
							onchange: move |event: FormEvent| {
								include_hidden_files.set(parse_bool(&event.data.value()));
							},
						}
					}
					// Include system files
					ConfigElement {
						id: "cfg_main_include_system_files",
						label: tid!("view_config_main_msg_include_system_files"),
						Checkbox {
							id: "cfg_main_include_system_files",
							name: "cfg_main_include_system_files",
							checked: include_system_files(),
							onchange: move |event: FormEvent| {
								include_system_files.set(parse_bool(&event.data.value()));
							},
						}
					}
					// Set files as read-only
					ConfigElement {
						id: "cfg_main_set_files_readonly",
						label: tid!("view_config_main_msg_set_files_readonly"),
						Checkbox {
							id: "cfg_main_set_files_readonly",
							name: "cfg_main_set_files_readonly",
							checked: set_files_readonly(),
							onchange: move |event: FormEvent| {
								set_files_readonly.set(parse_bool(&event.data.value()));
							},
						}
					}
				}
				ApplyConfig {
					onclick: move |_event| {
						let new_include_hidden_files = include_hidden_files();
						let new_include_system_files = include_system_files();
						let new_set_files_readonly = set_files_readonly();
						spawn(async move {
							let mut cfg = cfg_sig();
							cfg.include_hidden_files = Some(new_include_hidden_files);
							cfg.include_system_files = Some(new_include_system_files);
							cfg.set_files_as_readonly = Some(new_set_files_readonly);
							cfg.write_to_file();
							cfg_sig.set(cfg);
						});
					},
				}
			}
		}
	}
}
