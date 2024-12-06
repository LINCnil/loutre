#![allow(non_snake_case)]

use crate::components::{
	ConfigMenu, ConfigMenuHighlight, DropZone, Grid, Header, MainSection, Select, SelectOption,
};
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

	rsx! {
		DropZone {
			Header {
				is_config_view: true,
			}
			MainSection {
				h1 {
					{ t!("view_config_title") }
				}
			ConfigMenu { hl: ConfigMenuHighlight::Hash }
			Grid {
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
						selected_option: cfg_sig().hash_function.to_string().to_lowercase(),
						onchange: move |event: FormEvent| {
							if let Ok(new_value) = HashFunc::from_str(&event.data.value()) {
								spawn(async move {
									let mut cfg = cfg_sig();
									cfg.hash_function = new_value;
									cfg.write_to_file();
									cfg_sig.set(cfg);
								});
							}
						},
					}
				}

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
						selected_option: cfg_sig().content_file_format.get_value(),
						onchange: move |event: FormEvent| {
							if let Ok(new_value) = ContentFileFormat::from_str(&event.data.value()) {
								spawn(async move {
									let mut cfg = cfg_sig();
									cfg.content_file_format = new_value;
									cfg.write_to_file();
									cfg_sig.set(cfg);
								});
							}
						},
					}
				}

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
						value: cfg_sig().get_content_file_name(),
						disabled: true,
					}
				}
			}
		}
	}
	}
}
