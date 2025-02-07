#![allow(non_snake_case)]

use crate::app::Route;
use crate::clipboard::ClipboardDefaultTemplate;
use crate::components::{ApplyConfig, Button, Header, MainSection, Root};
use crate::config::Config;
use crate::templates::{filter_add_dir_level, filter_nb_letters, EntryTemplate};
use dioxus::prelude::*;
use dioxus_i18n::tid;
use minijinja::{context, Environment};

const EXAMPLE_NB_EVIDENCES: usize = 42;

#[component]
pub fn ClipboardTemplateConfig(tpl_id: usize) -> Element {
	let config = use_context::<Signal<Config>>()();
	let tpl = ClipboardDefaultTemplate::from_id(tpl_id).unwrap();
	let mut cfg_sig = use_context::<Signal<Config>>();
	let mut new_tpl = use_signal(|| tpl.get_template(&config, EXAMPLE_NB_EVIDENCES));
	let mut preview = use_signal(|| gen_preview(new_tpl.to_string()));

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::ClipboardConfig {}),
				h1 {
					{ tid!("view_config_title") }
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
					id: "view_config_clipboard_preview_area",
					dangerous_inner_html: "{preview}",
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
					{ tid!("view_config_clipboard_msg_reset_value") }
				}
			}
		}
	}
}

fn gen_preview(input: String) -> String {
	#[cfg(unix)]
	let base_dir = "test".as_bytes().to_vec();
	#[cfg(not(unix))]
	let base_dir = "test".to_string();
	#[cfg(unix)]
	let relative_path = "test".as_bytes().to_vec();
	#[cfg(not(unix))]
	let relative_path = "test".to_string();
	let mut env = Environment::new();
	env.add_filter("add_dir_level", filter_add_dir_level);
	env.add_filter("nb_letters", filter_nb_letters);
	let ctx = context!(
		hash_func => "SHA-256",
		nb_evidences => 3,
		nb_start => 1,
		evidence => context!(
			name => "test.txt",
			size => 42,
			hash => "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			hash_func => "SHA-256",
		),
		evidences => vec![
			EntryTemplate {
				base_dir: base_dir.clone(),
				relative_path: relative_path.clone(),
				name: "test".to_string(),
				is_dir: true,
				is_file: false,
				size: 2,
				hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
				hash_func: "SHA-256".to_string(),
				evidences: vec![
					EntryTemplate {
						base_dir: base_dir.clone(),
						relative_path: relative_path.clone(),
						name: "asdf.txt".to_string(),
						is_dir: false,
						is_file: true,
						size: 4,
						hash: "4ba5df017a3d79d1e4ed92a83e38c20eacaf76b42486a199c91ac5bbb70c73f4".to_string(),
						hash_func: "SHA-256".to_string(),
						evidences: Vec::new(),
					},
					EntryTemplate {
						base_dir: base_dir.clone(),
						relative_path: relative_path.clone(),
						name: "space.txt".to_string(),
						is_dir: false,
						is_file: true,
						size: 2,
						hash: "5fe719893f2a0957435a95aa75dbe82e637e3806805c2cef9bc5836baeaf9ff7".to_string(),
						hash_func: "SHA-256".to_string(),
						evidences: Vec::new(),
					},
				],
			},
			EntryTemplate {
				base_dir,
				relative_path,
				name: "some file.txt".to_string(),
				is_dir: false,
				is_file: true,
				size: 42,
				hash: "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string(),
				hash_func: "SHA-256".to_string(),
				evidences: Vec::new(),
			},
		],
	);
	if let Err(e) = env.add_template("preview", &input) {
		tracing::error!("Unable to load template: {e}");
		return String::new();
	}
	let tmpl = env.get_template("preview").unwrap();
	match tmpl.render(&ctx) {
		Ok(s) => s,
		Err(e) => {
			tracing::error!("Unable to render template: {e}");
			String::new()
		}
	}
}
