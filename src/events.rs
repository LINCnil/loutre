use crate::clipboard::Clipboard;
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
	pub fn handle(
		self,
		cfg_sig: Signal<Config>,
		mut clb_sig: Signal<Clipboard>,
		mut fl_sig: Signal<FileList>,
		mut lb_sig: Signal<LoadingBarStatus>,
		mut pg_sig: Signal<Option<ProgressBarStatus>>,
		mut rcpt_sig: Signal<Option<Receipt>>,
	) {
		match self {
			Self::FileListReset => {
				fl_sig.set(FileList::None);
			}
			Self::HashedFileListSet(new_hfl) => {
				let cfg = cfg_sig();
				let mut clipboard = Clipboard::new();
				clipboard.set_clipboard(&cfg, &new_hfl, cfg.get_clipboard_threshold());
				clb_sig.set(clipboard);
				fl_sig.set(FileList::Hashed(new_hfl));
			}
			Self::NonHashedFileListSet(new_fl) => {
				fl_sig.set(FileList::NonHashed(new_fl));
			}
			Self::LoadingBarAdd => {
				lb_sig.set(LoadingBarStatus::Displayed);
			}
			Self::LoadingBarDelete => {
				lb_sig.set(LoadingBarStatus::Hidden);
			}
			Self::ProgressBarAdd(nb) => match pg_sig() {
				Some(mut status) => {
					status.add_progress(nb);
					pg_sig.set(Some(status));
				}
				None => {
					error!("No active progress bar for ProgressBarAdd({nb})");
				}
			},
			Self::ProgressBarCreate(nb) => {
				pg_sig.set(Some(ProgressBarStatus::new(nb)));
			}
			Self::ProgressBarDelete => {
				pg_sig.set(None);
			}
			Self::ReceiptReset => {
				rcpt_sig.set(None);
			}
			Self::ReceiptSet(rcpt) => {
				rcpt_sig.set(Some(rcpt));
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
