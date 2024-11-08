#![allow(non_snake_case)]

use crate::components::{
	Button, DropZone, FileButton, FileListIndicator, FileListReceipt, Header, LoadingBar,
	NotificationList, ProgressBar,
};
use crate::config::Config;
use crate::events::{send_event, send_event_sync, ExternalEvent, ExternalEventSender};
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
	let file_list_sig = use_context::<Signal<FileList>>();
	let pg_status_opt = use_context::<Signal<Option<ProgressBarStatus>>>()();
	let config_sig = use_context::<Signal<Config>>();
	let receipt_opt_sig = use_context::<Signal<Option<Receipt>>>();
	let tx_sig = use_context::<Signal<ExternalEventSender>>();
	rsx! {
		DropZone {
			ondrop: move |event: DragEvent| {
				info!("DragEvent received: {event:?}");
				spawn(async move {
					if let Some(file_engine) = event.files() {
						load_file(&config_sig(), tx_sig(), file_engine).await;
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
								load_file(&config_sig(), tx_sig(), file_engine).await;
							}
						});
					},
					{ t!("view_main_open_dir") }
				}
				FileButton {
					icon: "ri-mail-check-line",
					accept: ".msg,.txt",
					multiple: false,
					directory: false,
					name: "view-main-btn-select-receipt",
					onchange: move |event: FormEvent| {
						spawn(async move {
							if let Some(file_engine) = event.files() {
								load_file(&config_sig(), tx_sig(), file_engine).await;
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
					if let FileList::NonHashed(_) = file_list_sig() {
						Button {
							onclick: move |_event| {
								spawn(async move {
									calc_fingerprints(&config_sig(), tx_sig(), receipt_opt_sig(), file_list_sig()).await;
								});
							},
							{ t!("view_main_calc_fingerprints") }
						}
					}
					if let FileList::Hashed(_) = file_list_sig() {
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

async fn load_file(config: &Config, tx: ExternalEventSender, file_engine: Arc<dyn FileEngine>) {
	info!("File loading: {:?}", file_engine.files());
	if let Some(f) = file_engine.files().first() {
		let path = Path::new(f);
		if path.is_file() {
			load_receipt(config, tx.clone(), path).await;
		}
		if path.is_dir() {
			load_directory(config, tx, path).await;
		}
	}
}

async fn load_directory(config: &Config, tx: ExternalEventSender, path: &Path) {
	info!(
		"Directory loading async function started: {}",
		path.display()
	);
	let include_hidden_files = config.include_hidden_files();
	let include_system_files = config.include_system_files();
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

async fn load_receipt(config: &Config, tx: ExternalEventSender, path: &Path) {
	info!("Loading receipt: {}", path.display());
	let default_hash = match crate::analyse_hash::from_path(path) {
		Some(h) => h,
		None => config.hash_function,
	};
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

async fn calc_fingerprints(
	config: &Config,
	tx: ExternalEventSender,
	receipt_opt: Option<Receipt>,
	file_list: FileList,
) {
	info!("File hashing async function started");
	let hash_func = match &receipt_opt {
		Some(rcpt) => rcpt.get_main_hashing_function(),
		None => config.hash_function,
	};

	if let FileList::NonHashed(file_list) = file_list {
		thread::spawn(move || {
			info!("File hashing thread started");

			let total_size = file_list.total_size();
			send_event_sync(&tx, ExternalEvent::ProgressBarCreate(total_size));
			info!("Total size to hash: {total_size} bytes");

			match file_list.hash(hash_func, tx.clone()) {
				Ok(hashed_file_list) => {
					send_event_sync(&tx, ExternalEvent::HashedFileListSet(hashed_file_list));
				}
				Err(e) => error!("Unable to hash files: {e}"),
			};
			send_event_sync(&tx, ExternalEvent::ProgressBarDelete);

			if let Some(rcpt) = receipt_opt {
				info!("Checking fingerprints against the receipt");
				// TODO
			}

			info!("File hashing thread done");
		});
	}

	info!("File hashing async function done");
}

async fn check_fingerprints() {
	info!("Data integrity check async function started");
	// TODO
	info!("Data integrity check async function done");
}
