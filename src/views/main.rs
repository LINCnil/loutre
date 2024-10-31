#![allow(non_snake_case)]

use crate::components::{
	DropZone, FileButton, FileListIndicator, FileListReceipt, Header, LoadingBar, NotificationList,
	ProgressBar,
};
use crate::events::{ExternalEvent, ExternalEventSender};
use crate::files::{FileList, NonHashedFileList};
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
	rsx! {
		DropZone {
			ondrop: move |event: DragEvent| {
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
	let mut fl_sig = use_context::<Signal<FileList>>();
	fl_sig.set(FileList::None);
	let pg_tx = use_context::<Signal<ExternalEventSender>>()();
	let handle = Handle::current();
	let path = path.to_path_buf();

	thread::spawn(move || {
		handle.spawn(async move {
			info!("Directory loading thread started");
			if let Err(e) = pg_tx.send(ExternalEvent::LoadingBarAdd).await {
				error!("Error sending loading bar message: {e}");
			}
			match NonHashedFileList::from_dir(&path).await {
				Ok(new_lst) => {
					if let Err(e) = pg_tx
						.send(ExternalEvent::NonHashedFileListSet(new_lst))
						.await
					{
						error!("Error sending non-hashed file list set message: {e}");
					}
				}
				Err(e) => error!("Unable to load directory: {}: {e}", path.display()),
			};
			if let Err(e) = pg_tx.send(ExternalEvent::LoadingBarDelete).await {
				error!("Error sending loading bar message: {e}");
			}
			info!("Directory loading thread done");
		});
	});
	info!("Directory loading async function done");
}

async fn load_receipt(path: &Path) {
	info!("Loading receipt: {}", path.display());
}
