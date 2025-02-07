#![allow(non_snake_case)]

use crate::events::{send_event, ExternalEvent, ExternalEventSender};
use crate::files::FileList;
use crate::receipt::Receipt;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn FileListIndicator() -> Element {
	rsx! {
		FileListMeta {
			is_receipt: false,
		}
	}
}

#[component]
pub fn FileListReceipt() -> Element {
	rsx! {
		FileListMeta {
			is_receipt: true,
		}
	}
}

#[component]
fn FileListMeta(is_receipt: bool) -> Element {
	let receipt_opt_sig = use_context::<Signal<Option<Receipt>>>();
	let file_list = use_context::<Signal<FileList>>()();
	let path_opt = if is_receipt {
		receipt_opt_sig().map(|rcpt| rcpt.to_string())
	} else {
		match file_list {
			FileList::NonHashed(lst) => Some(lst.get_base_dir().display().to_string()),
			FileList::Hashed(lst) => Some(lst.get_base_dir().display().to_string()),
			FileList::None => None,
		}
	};
	rsx! {
		if let Some(path) = path_opt {
			FileListIndicatorElement {
				path: "{path}",
				is_receipt: is_receipt,
			}
		}
	}
}

#[component]
fn FileListIndicatorElement(path: String, is_receipt: bool) -> Element {
	let tx = use_context::<Signal<ExternalEventSender>>()();
	let icon_class = if is_receipt {
		"ri-mail-check-line"
	} else {
		"ri-folder-5-line"
	};
	rsx! {
		p {
			class: "component-file-list",
			span {
				class: "component-file-list-icon {icon_class}",
			}
			span {
				class: "component-file-list-content",
				"{path}"
			}
			span {
				class: "component-file-list-delete ri-close-large-line",
				title: tid!("cpn_file_list_delete"),
				onclick: move |_| {
					let txc = tx.clone();
					spawn(async move {
						if is_receipt {
							send_event(&txc, ExternalEvent::ReceiptReset);
							tracing::info!("Removing receipt");
						} else {
							tracing::info!("Removing file list");
							send_event(&txc, ExternalEvent::FileListReset);
						}
					});
				},
			}
		}
	}
}
