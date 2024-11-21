use crate::clipboard::{Clipboard, ClipboardStart};
use crate::config::Config;
use crate::files::{FileList, HashedFileList, NonHashedFileList};
use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use crate::receipt::Receipt;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use tokio::sync::mpsc::{Receiver, Sender};

pub type ExternalEventReceiver = Receiver<ExternalEvent>;
pub type ExternalEventSender = Sender<ExternalEvent>;

pub async fn send_event(tx: &ExternalEventSender, event: ExternalEvent) -> bool {
	if let Err(e) = tx.send(event).await {
		error!("Error sending event: {e}");
		return false;
	}
	true
}

pub fn send_event_sync(tx: &ExternalEventSender, event: ExternalEvent) -> bool {
	if let Err(e) = tx.blocking_send(event) {
		error!("Error sending event: {e}");
		return false;
	}
	true
}

pub struct ExternalEventSignals {
	config: Signal<Config>,
	clipboard: Signal<Clipboard>,
	clipboard_start: Signal<ClipboardStart>,
	file_list: Signal<FileList>,
	loading_bar: Signal<LoadingBarStatus>,
	progress_bar: Signal<Option<ProgressBarStatus>>,
	receipt: Signal<Option<Receipt>>,
}

impl ExternalEventSignals {
	pub fn new() -> Self {
		Self {
			config: use_context::<Signal<Config>>(),
			clipboard: use_context::<Signal<Clipboard>>(),
			clipboard_start: use_context::<Signal<ClipboardStart>>(),
			file_list: use_context::<Signal<FileList>>(),
			loading_bar: use_context::<Signal<LoadingBarStatus>>(),
			progress_bar: use_context::<Signal<Option<ProgressBarStatus>>>(),
			receipt: use_context::<Signal<Option<Receipt>>>(),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ExternalEvent {
	FileListReset,
	HashedFileListSet(HashedFileList),
	NonHashedFileListSet(NonHashedFileList),
	LoadingBarAdd,
	LoadingBarDelete,
	ProgressBarAdd(u64),
	ProgressBarCreate(u64),
	ProgressBarDelete,
	ReceiptReset,
	ReceiptSet(Receipt),
}

impl ExternalEvent {
	pub fn handle(self, signals: &mut ExternalEventSignals) {
		match self {
			Self::FileListReset => {
				signals.file_list.set(FileList::None);
			}
			Self::HashedFileListSet(new_hfl) => {
				if new_hfl.get_result().is_ok() {
					let cfg = (signals.config)();
					let mut clipboard = Clipboard::new();
					clipboard.set_clipboard(
						&cfg,
						&new_hfl,
						(signals.clipboard_start)(),
						cfg.get_clipboard_threshold(),
					);
					signals.clipboard.set(clipboard);
				}
				signals.file_list.set(FileList::Hashed(new_hfl));
			}
			Self::NonHashedFileListSet(new_fl) => {
				signals.file_list.set(FileList::NonHashed(new_fl));
			}
			Self::LoadingBarAdd => {
				signals.loading_bar.set(LoadingBarStatus::Displayed);
			}
			Self::LoadingBarDelete => {
				signals.loading_bar.set(LoadingBarStatus::Hidden);
			}
			Self::ProgressBarAdd(nb) => match (signals.progress_bar)() {
				Some(mut status) => {
					status.add_progress(nb);
					signals.progress_bar.set(Some(status));
				}
				None => {
					error!("No active progress bar for ProgressBarAdd({nb})");
				}
			},
			Self::ProgressBarCreate(nb) => {
				signals.progress_bar.set(Some(ProgressBarStatus::new(nb)));
			}
			Self::ProgressBarDelete => {
				signals.progress_bar.set(None);
			}
			Self::ReceiptReset => {
				signals.receipt.set(None);
			}
			Self::ReceiptSet(rcpt) => {
				signals.receipt.set(Some(rcpt));
			}
		}
	}
}

impl std::fmt::Display for ExternalEvent {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::NonHashedFileListSet(lst) => write!(
				f,
				"NonHashedFileListSet(NonHashedFileList {{ {} elements }})",
				lst.len()
			),
			_ => write!(f, "{:?}", self),
		}
	}
}
