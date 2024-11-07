#![allow(non_snake_case)]

use crate::components::{
	Button, DropZone, FileButton, FileListIndicator, FileListReceipt, Header, LoadingBar,
	NotificationList, ProgressBar,
};
use crate::config::Config;
use crate::events::{send_event, ExternalEvent, ExternalEventSender};
use crate::files::{FileList, NonHashedFileList};
use crate::progress::ProgressBarStatus;
use crate::receipt::Receipt;
use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_logger::tracing::{error, info};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Handle;

#[component]
pub fn Main() -> Element {
	let file_list = use_context::<Signal<FileList>>()();
	let pg_status_opt = use_context::<Signal<Option<ProgressBarStatus>>>()();
	rsx! {
		DropZone {
			ondrop: move |event: DragEvent| {
				info!("DragEvent received: {event:?}");
				spawn(async move {
					if let Some(file_engine) = event.files() {
						load_file(file_engine).await;
					}
				});
			},
			Header {}
			div {
				FileButton {
					icon: "ri-folder-5-line",
					accept: "",
					multiple: false,
					directory: true,
					name: "view-main-btn-select-directory",
					onchange: move |event: FormEvent| {
						spawn(async move {
							if let Some(file_engine) = event.files() {
								load_file(file_engine).await;
							}
						});
					},
					{ t!("view_main_open_dir") }
				}
				FileButton {
					icon: "ri-mail-check-line",
					accept: ".msg, .txt",
					multiple: false,
					directory: false,
					name: "view-main-btn-select-receipt",
					onchange: move |event: FormEvent| {
						spawn(async move {
							if let Some(file_engine) = event.files() {
								load_file(file_engine).await;
							}
						});
					},
					{ t!("view_main_open_receipt") }
				}
			}
			FileListIndicator {}
			FileListReceipt {}
			NotificationList {}
			ProgressBar {}
			LoadingBar {}

			if pg_status_opt.is_none() {
				div {
					if let FileList::NonHashed(_) = file_list {
						Button {
							onclick: move |_event| {
								spawn(async move {
									calc_fingerprints().await;
								});
							},
							{ t!("view_main_calc_fingerprints") }
						}
					}
					if let FileList::Hashed(_) = file_list {
						Button {
							onclick: move |_event| {
								spawn(async move {
									check_fingerprints().await;
								});
							},
							{ t!("view_main_check_fingerprints") }
						}
					}
				}
			}
		}
	}
}

async fn load_file(file_engine: Arc<dyn FileEngine>) {
	info!("File loading: {:?}", file_engine.files());
	if let Some(f) = file_engine.files().first() {
		let path = Path::new(f);
		if path.is_dir() {
			load_directory(path).await;
		}
		if path.is_file() {
			load_receipt(path).await;
		}
	}
}

async fn load_directory(path: &Path) {
	info!("Loading directory: {}", path.display());
	let config = use_context::<Signal<Config>>()();
	let include_hidden_files = config.include_hidden_files();
	let include_system_files = config.include_system_files();
	let tx = use_context::<Signal<ExternalEventSender>>()();
	send_event(&tx, ExternalEvent::FileListReset).await;
	let handle = Handle::current();
	let path = path.to_path_buf();

	thread::spawn(move || {
		handle.spawn(async move {
			info!("Directory loading thread started");
			send_event(&tx, ExternalEvent::LoadingBarAdd).await;
			match NonHashedFileList::from_dir(&path, include_hidden_files, include_system_files)
				.await
			{
				Ok(new_lst) => {
					send_event(&tx, ExternalEvent::NonHashedFileListSet(new_lst)).await;
				}
				Err(e) => error!("Unable to load directory: {}: {e}", path.display()),
			};
			send_event(&tx, ExternalEvent::LoadingBarDelete).await;
			info!("Directory loading thread done");
		});
	});
	info!("Directory loading async function done");
}

async fn load_receipt(path: &Path) {
	info!("Loading receipt: {}", path.display());
	let default_hash = crate::hash::HashFunc::Sha256; // TODO: REMOVE ME
	let tx = use_context::<Signal<ExternalEventSender>>()();
	send_event(&tx, ExternalEvent::ReceiptReset).await;
	let handle = Handle::current();
	let path = path.to_path_buf();

	thread::spawn(move || {
		handle.spawn(async move {
			info!("Receipt loading thread started");
			send_event(&tx, ExternalEvent::LoadingBarAdd).await;
			match Receipt::new(&path, default_hash) {
				Ok(new_receipt) => {
					send_event(&tx, ExternalEvent::ReceiptSet(new_receipt)).await;
				}
				Err(_) => error!("Unable to load receipt: {}", path.display()),
			};
			send_event(&tx, ExternalEvent::LoadingBarDelete).await;
			info!("Receipt loading thread done");
		});
	});
	info!("Receipt loading async function done");
}

async fn calc_fingerprints() {
	info!("Starting file hashing");
	// TODO: hash the files
	// TODO: if there is a receipt, check fingerprints against it
	// info!("Checking fingerprints against the receipt");
}

async fn check_fingerprints() {
	info!("Starting data integrity check");
	// TODO
}
