use crate::files::{FileList, NonHashedFileList};
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

#[derive(Clone, Debug)]
pub enum ExternalEvent {
	FileListReset,
	NonHashedFileListSet(NonHashedFileList),
	LoadingBarAdd,
	LoadingBarDelete,
	ProgressBarAdd(usize),
	ProgressBarCreate(usize),
	ProgressBarDelete,
	ReceiptReset,
	ReceiptSet(Receipt),
}

impl ExternalEvent {
	pub fn handle(
		self,
		mut fl_sig: Signal<FileList>,
		mut lb_sig: Signal<LoadingBarStatus>,
		mut pg_sig: Signal<Option<ProgressBarStatus>>,
		mut rcpt_sig: Signal<Option<Receipt>>,
	) {
		match self {
			Self::FileListReset => {
				fl_sig.set(FileList::None);
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
