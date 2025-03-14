#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::config::ConfigElement;
use crate::components::{Button, Header, MainSection, Root};
use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn ContentFileNameConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut new_name = use_signal(|| cfg_sig().content_file_name.unwrap_or_default());

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::HashConfig {}),
				h1 {
					{ tid!("view_config_title") }
				}
				form {
					// Custom file name
					ConfigElement {
						id: "cfg_ctn_file_name_custom",
						label: tid!("view_config_ctn_file_name_custom_value"),
						input {
							id: "cfg_ctn_file_name_custom",
							name: "cfg_ctn_file_name_custom",
							value: "{new_name}",
							oninput: move |event| new_name.set(event.value())
						}
						Button {
							onclick: move |_event| {
								let new_name_val = new_name();
								if !new_name_val.is_empty() {
									spawn(async move {
										let mut cfg = cfg_sig();
										cfg.content_file_name = Some(new_name_val);
										cfg.write_to_file();
										cfg_sig.set(cfg);
										navigator().push(Route::HashConfig {});
									});
								}
							},
							{ tid!("cpn_form_apply_config") }
						}
					}
					// Default file name
					ConfigElement {
						id: "cfg_ctn_file_name_default",
						label: tid!("view_config_ctn_file_name_default_value"),
						Button {
							onclick: move |_event| {
								spawn(async move {
									let mut cfg = cfg_sig();
									cfg.content_file_name = None;
									cfg.write_to_file();
									cfg_sig.set(cfg);
									navigator().push(Route::HashConfig {});
								});
							},
							{ tid!("cpn_form_apply_config") }
						}
					}
				}
			}
		}
	}
}
