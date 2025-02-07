#![allow(non_snake_case)]

use crate::app::Route;
use crate::clipboard::{ClipboardDefaultTemplate, ClipboardPersistence, ClipboardStart};
use crate::components::config::{ConfigElement, ConfigMenu, ConfigMenuHighlight};
use crate::components::{ApplyConfig, Button, Header, MainSection, Root, Select, SelectOption};
use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use std::str::FromStr;

#[component]
pub fn ClipboardConfig() -> Element {
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut clipboard_start_sig = use_context::<Signal<ClipboardStart>>();
	let cl_pers_opts = vec![
		SelectOption::new(
			tid!("view_config_clipboard_msg_persistence_default"),
			ClipboardPersistence::Default.to_string(),
		),
		SelectOption::new(
			tid!("view_config_clipboard_msg_persistence_activated"),
			ClipboardPersistence::Activated.to_string(),
		),
		SelectOption::new(
			tid!("view_config_clipboard_msg_persistence_deactivated"),
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
					{ tid!("view_config_title") }
				}
				ConfigMenu { hl: ConfigMenuHighlight::Clipboard }
				form {
					// First evidence number
					ConfigElement {
						id: "cfg_clipboard_clipboard_start",
						label: tid!("view_config_clipboard_start_msg"),
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
					// Cliboard threshold
					ConfigElement {
						id: "cfg_clipboard_threshold",
						label: tid!("view_config_clipboard_msg_threshold"),
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
					// Cliboard persistence
					ConfigElement {
						id: "cfg_clipboard_persistence",
						label: tid!("view_config_clipboard_msg_persistence"),
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
					// Cliboard template: HTML list
					ClipboardTemplateEdit {
						id: "cfg_clipboard_tpl_list_html",
						label: "view_config_clipboard_msg_tpl_list_html",
						tpl_id: ClipboardDefaultTemplate::ListHtml as usize,
						has_default: cfg_sig().clipboard_tpl_html_list.is_none(),
					}
					// Cliboard template: TXT list
					ClipboardTemplateEdit {
						id: "cfg_clipboard_tpl_list_txt",
						label: "view_config_clipboard_msg_tpl_list_txt",
						tpl_id: ClipboardDefaultTemplate::ListText as usize,
						has_default: cfg_sig().clipboard_tpl_txt_list.is_none(),
					}
					// Cliboard template: HTML content file
					ClipboardTemplateEdit {
						id: "cfg_clipboard_tpl_ctn_file_html",
						label: "view_config_clipboard_msg_tpl_ctn_file_html",
						tpl_id: ClipboardDefaultTemplate::ContentFileHtml as usize,
						has_default: cfg_sig().clipboard_tpl_html_ctn_file.is_none(),
					}
					// Cliboard template: TXT content file
					ClipboardTemplateEdit {
						id: "cfg_clipboard_tpl_ctn_file_txt",
						label: "view_config_clipboard_msg_tpl_ctn_file_txt",
						tpl_id: ClipboardDefaultTemplate::ContentFileText as usize,
						has_default: cfg_sig().clipboard_tpl_txt_ctn_file.is_none(),
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

#[derive(Clone, PartialEq, Props)]
struct ClipboardTemplateEditProps {
	id: String,
	label: String,
	tpl_id: usize,
	has_default: bool,
}

#[component]
fn ClipboardTemplateEdit(props: ClipboardTemplateEditProps) -> Element {
	rsx! {
		ConfigElement {
			id: props.id,
			label: tid!(&props.label),
			span {
				class: "view-config-clipboard-msg-spacer",
				if props.has_default {
					{ tid!("view_config_clipboard_msg_has_default_value") }
				} else {
					{ tid!("view_config_clipboard_msg_has_custom_value") }
				}
			}
			Button {
				onclick: move |_event| {
					navigator().push(Route::ClipboardTemplateConfig { tpl_id: props.tpl_id });
				},
				{ tid!("view_config_clipboard_msg_edit_value") }
			}
		}
	}
}
