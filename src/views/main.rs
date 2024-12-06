#![allow(non_snake_case)]

use crate::check::{check, CheckResult, CheckResultError, CheckType};
use crate::clipboard::{Clipboard, ClipboardStart};
use crate::components::{
	Button, DropZone, FileButton, FileListIndicator, FileListReceipt, Header, LoadingBar,
	MainSection, Notification, NotificationList, ProgressBar,
};
use crate::config::Config;
use crate::events::{send_event, send_event_sync, ExternalEvent, ExternalEventSender};
use crate::files::{FileList, NonHashedFileList};
use crate::notifications::NotificationLevel;
use crate::progress::{LoadingBarStatus, ProgressBarStatus};
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
	let lb_status = use_context::<Signal<LoadingBarStatus>>()();
	let config_sig = use_context::<Signal<Config>>();
	let receipt_opt_sig = use_context::<Signal<Option<Receipt>>>();
	let tx_sig = use_context::<Signal<ExternalEventSender>>();
	let mut clipboard_sig = use_context::<Signal<Clipboard>>();
	let clipboard_start_sig = use_context::<Signal<ClipboardStart>>();

	let has_progress_bar = pg_status_opt.is_some();
	let has_loading_bar = lb_status == LoadingBarStatus::Displayed;
	let is_waiting = has_progress_bar || has_loading_bar;

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
			MainSection {
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
						if let FileList::NonHashed(file_lst) = file_list_sig() {
							if !is_waiting {
								if file_lst.content_file_exists(&config_sig()) {
									Button {
										icon: "ri-shield-check-line",
										onclick: move |_event| {
											spawn(async move {
												calc_fingerprints(&config_sig(), tx_sig(), receipt_opt_sig(), file_list_sig()).await;
											});
										},
										{ t!("view_main_check_fingerprints") }
									}
								} else {
									Button {
										icon: "ri-shield-flash-line",
										onclick: move |_event| {
											spawn(async move {
												calc_fingerprints(&config_sig(), tx_sig(), receipt_opt_sig(), file_list_sig()).await;
											});
										},
										{ t!("view_main_calc_fingerprints") }
									}
								}
							}
						}
						if let FileList::Hashed(lst) = file_list_sig() {
							if let CheckResult::Ok = lst.get_result() {
								Notification {
									id: "view-main-file-check-ok",
									level: NotificationLevel::Success,
									title: t!("view_main_check_result_title"),
									p { { t!("view_main_check_result_ok_text") } }
								}
								Button {
									icon: "ri-clipboard-line",
									onclick: move |_event| {
										if let FileList::Hashed(lst) = file_list_sig() {
											let mut clipboard = Clipboard::new();
											println!("- debug: clipboard_start_sig: {}", clipboard_start_sig());
											let _ = clipboard.set_clipboard_list(
												&config_sig(),
												&lst,
												clipboard_start_sig(),
											);
											clipboard_sig.set(clipboard);
										}
									},
								}
								Button {
									icon: "ri-file-copy-2-line",
									onclick: move |_event| {
										if let FileList::Hashed(lst) = file_list_sig() {
											let cfg = config_sig();
											let mut clipboard = Clipboard::new();
											let _ = clipboard.set_clipboard_ctn_file(
												&cfg,
												&lst,
												clipboard_start_sig(),
											);
											clipboard_sig.set(clipboard);
										}
									},
								}
							}
							if let CheckResult::Error(_) = lst.get_result() {
								Notification {
									id: "view-main-file-check-err",
									level: NotificationLevel::Error,
									title: t!("view_main_check_result_title"),
									p { { t!("view_main_check_result_err_text") } }
								}
							}
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
	let config = config.clone();

	if let FileList::NonHashed(file_list) = file_list {
		thread::spawn(move || {
			info!("File hashing thread started");

			let total_size = file_list.total_size();
			send_event_sync(&tx, ExternalEvent::ProgressBarCreate(total_size));
			info!("Total size to hash: {total_size} bytes");

			// Calculating fingerprints
			match file_list.hash(&config, hash_func, tx.clone()) {
				Ok(mut hashed_file_list) => {
					send_event_sync(&tx, ExternalEvent::ProgressBarDelete);
					send_event_sync(&tx, ExternalEvent::LoadingBarAdd);

					// Checking fingerprints against the content file
					info!("Checking fingerprints against the content file");
					if let Ok(ctn_file_path) =
						hashed_file_list.get_content_file_absolute_path(&config)
					{
						let default_hash = match crate::analyse_hash::from_path(&ctn_file_path) {
							Some(h) => h,
							None => config.hash_function,
						};
						match Receipt::new(&ctn_file_path, default_hash) {
							Ok(ctn_file) => {
								match check(
									&hashed_file_list,
									ctn_file.get_file_list(),
									CheckType::ContentFile,
								) {
									CheckResult::Ok => hashed_file_list.set_result_ok(),
									CheckResult::Error(err_lst) => {
										for e in err_lst {
											hashed_file_list.push_result_error(e);
										}
									}
									CheckResult::None => {}
								}
							}
							Err(_) => {
								hashed_file_list
									.push_result_error(CheckResultError::ContentFileParseError);
							}
						};
					}

					// Checking fingerprints against the receipt
					if let Some(rcpt) = receipt_opt {
						info!("Checking fingerprints against the receipt");
						match check(&hashed_file_list, rcpt.get_file_list(), CheckType::Receipt) {
							CheckResult::Ok => {
								if !hashed_file_list.get_result().is_err() {
									hashed_file_list.set_result_ok()
								}
							}
							CheckResult::Error(err_lst) => {
								for e in err_lst {
									hashed_file_list.push_result_error(e);
								}
							}
							CheckResult::None => {}
						}
					}

					send_event_sync(&tx, ExternalEvent::HashedFileListSet(hashed_file_list));
					send_event_sync(&tx, ExternalEvent::LoadingBarDelete);
				}
				Err(e) => error!("Unable to hash files: {e}"),
			};

			info!("File hashing thread done");
		});
	}

	info!("File hashing async function done");
}
