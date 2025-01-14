#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::config::{ConfigElement, ConfigMenu, ConfigMenuHighlight};
use crate::components::{ApplyConfig, Header, MainSection, Root, Select, SelectOption};
use crate::config::Config;
use crate::content_file_format::ContentFileFormat;
use crate::hash::HashFunc;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[component]
pub fn HashConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
	let hash_func_opts = HashFunc::iter()
		.map(|h| SelectOption::new(h.to_string(), h.to_string().to_lowercase()))
		.collect();
	let ctn_file_format_opts = ContentFileFormat::iter()
		.map(|h| SelectOption::new(h.to_string(), h.get_value()))
		.collect();
	let mut hash_function = use_signal(|| cfg_sig().hash_function);
	let mut content_file_format = use_signal(|| cfg_sig().content_file_format);

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ t!("view_config_title") }
				}
				ConfigMenu { hl: ConfigMenuHighlight::Hash }
				form {
					ConfigElement {
						// Hashing function
						p {
							label {
								r#for: "cfg_hash_algorithm",
								{ t!("view_config_hash_msg_hash_func") }
							}
						}
						div {
							Select {
								id: "cfg_hash_algorithm",
								name: "cfg_hash_algorithm",
								options: hash_func_opts,
								selected_option: hash_function().to_string().to_lowercase(),
								onchange: move |event: FormEvent| {
									if let Ok(new_value) = HashFunc::from_str(&event.data.value()) {
										hash_function.set(new_value);
									}
								},
							}
						}
					}
					ConfigElement {
						// Content file format
						p {
							label {
								r#for: "cfg_hash_content_file_format",
								{ t!("view_config_hash_msg_content_file_format") }
							}
						}
						div {
							Select {
								id: "cfg_hash_content_file_format",
								name: "cfg_hash_content_file_format",
								options: ctn_file_format_opts,
								selected_option: content_file_format().get_value(),
								onchange: move |event: FormEvent| {
									if let Ok(new_value) = ContentFileFormat::from_str(&event.data.value()) {
										content_file_format.set(new_value);
									}
								},
							}
						}
					}
					ConfigElement {
						// Content file name
						// TODO
						p {
							label {
								r#for: "cfg_hash_content_file_name",
								{ t!("view_config_hash_msg_content_file_name") }
							}
						}
						div {
							input {
								id: "cfg_hash_content_file_name",
								name: "cfg_hash_content_file_name",
								value: {
									let mut cfg = cfg_sig();
									cfg.content_file_format = content_file_format();
									cfg.get_content_file_name()
								},
								disabled: true,
							}
						}
					}
				}
				ApplyConfig {
					onclick: move |_event| {
						let new_hash_function = hash_function();
						let new_content_file_format = content_file_format();
						spawn(async move {
							let mut cfg = cfg_sig();
							cfg.hash_function = new_hash_function;
							cfg.content_file_format = new_content_file_format;
							cfg.write_to_file();
							cfg_sig.set(cfg);
						});
					},
				}
			}
		}
	}
}
