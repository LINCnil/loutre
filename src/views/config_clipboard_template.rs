#![allow(non_snake_case)]

use crate::app::Route;
use crate::clipboard::ClipboardDefaultTemplate;
use crate::components::{ApplyConfig, Button, Header, MainSection, Root};
use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::t;

const EXAMPLE_NB_EVIDENCES: usize = 42;

#[component]
pub fn ClipboardTemplateConfig(tpl_id: usize) -> Element {
	let config = use_context::<Signal<Config>>()();
	let tpl = ClipboardDefaultTemplate::from_id(tpl_id).unwrap();
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut new_tpl = use_signal(|| tpl.get_template(&config, EXAMPLE_NB_EVIDENCES));
	let mut preview = use_signal(|| new_tpl());

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::ClipboardConfig {}),
				h1 {
					{ t!("view_config_title") }
				}
				form {
					textarea {
						id: "view_config_clipboard_template_area",
						name: "view_config_clipboard_template_area",
						rows: 12,
						value: "{new_tpl}",
						oninput: move |event| {
							new_tpl.set(event.value());
							preview.set(gen_preview(event.value()));
						},
					}
				}
				div {
					dangerous_inner_html: preview,
				}
				ApplyConfig {
					onclick: move |_event| {
						spawn(async move {
							let mut cfg = cfg_sig();
							match tpl {
								ClipboardDefaultTemplate::ContentFileHtml => {
									cfg.clipboard_tpl_html_ctn_file = Some(new_tpl());
								},
								ClipboardDefaultTemplate::ContentFileText => {
									cfg.clipboard_tpl_txt_ctn_file = Some(new_tpl());
								},
								ClipboardDefaultTemplate::ListHtml => {
									cfg.clipboard_tpl_html_list = Some(new_tpl());
								},
								ClipboardDefaultTemplate::ListText => {
									cfg.clipboard_tpl_txt_list = Some(new_tpl());
								},
							};
							cfg.write_to_file();
							cfg_sig.set(cfg);
						});
					},
				}
				Button {
					onclick: move |_event| {
						spawn(async move {
							let mut cfg = cfg_sig();
							match tpl {
								ClipboardDefaultTemplate::ContentFileHtml => {
									cfg.clipboard_tpl_html_ctn_file = None;
								},
								ClipboardDefaultTemplate::ContentFileText => {
									cfg.clipboard_tpl_txt_ctn_file = None;
								},
								ClipboardDefaultTemplate::ListHtml => {
									cfg.clipboard_tpl_html_list = None;
								},
								ClipboardDefaultTemplate::ListText => {
									cfg.clipboard_tpl_txt_list = None;
								},
							};
							cfg.write_to_file();
							cfg_sig.set(cfg);
							navigator().push(Route::ClipboardConfig {});
						});
					},
					{ t!("view_config_clipboard_msg_reset_value") }
				}
			}
		}
	}
}

fn gen_preview(input: String) -> String {
	// TODO
	input
}
