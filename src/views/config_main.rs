#![allow(non_snake_case)]

use crate::components::{Checkbox, ConfigMenu, ConfigMenuHighlight, DropZone, Grid, Header};
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
				// Include hidden files
				p {
					label {
						r#for: "cfg_main_include_hidden_files",
						{ t!("view_config_main_msg_include_hidden_files") }
					}
				}
				div {
					Checkbox {
						id: "cfg_main_include_hidden_files",
						checked: cfg_sig().include_hidden_files(),
						onchange: move |event: FormEvent| {
							let new_value = parse_bool(&event.data.value());
							spawn(async move {
								let mut cfg = cfg_sig();
								cfg.include_hidden_files = Some(new_value);
								cfg.write_to_file();
								cfg_sig.set(cfg);
							});
						},
					}
				}

				// Include system files
				p {
					label {
						r#for: "cfg_main_include_system_files",
						{ t!("view_config_main_msg_include_system_files") }
					}
				}
				div {
					Checkbox {
						id: "cfg_main_include_system_files",
						checked: cfg_sig().include_system_files(),
						onchange: move |event: FormEvent| {
							let new_value = parse_bool(&event.data.value());
							spawn(async move {
								let mut cfg = cfg_sig();
								cfg.include_system_files = Some(new_value);
								cfg.write_to_file();
								cfg_sig.set(cfg);
							});
						},
					}
				}

				// Empty files warning
				p {
					label {
						r#for: "cfg_main_empty_files_warning",
						{ t!("view_config_main_msg_empty_files_warning") }
					}
				}
				div {
					Checkbox {
						id: "cfg_main_empty_files_warning",
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

				// Duplicated files warning
				p {
					label {
						r#for: "cfg_main_duplicated_files_warning",
						{ t!("view_config_main_msg_duplicated_files_warning") }
					}
				}
				div {
					Checkbox {
						id: "cfg_main_duplicated_files_warning",
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

				// Set files as read-only
				p {
					label {
						r#for: "cfg_main_set_files_readonly",
						{ t!("view_config_main_msg_set_files_readonly") }
					}
				}
				div {
					Checkbox {
						id: "cfg_main_set_files_readonly",
						checked: cfg_sig().set_files_as_readonly(),
						onchange: move |event: FormEvent| {
							let new_value = parse_bool(&event.data.value());
							spawn(async move {
								let mut cfg = cfg_sig();
								cfg.set_files_as_readonly = Some(new_value);
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
