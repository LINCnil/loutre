use crate::files::{FileList, NonHashedFileList};
use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use tokio::sync::mpsc::{Receiver, Sender};

pub type ExternalEventReceiver = Receiver<ExternalEvent>;
pub type ExternalEventSender = Sender<ExternalEvent>;

pub async fn send_event_full(event: ExternalEvent) -> bool {
	let tx = use_context::<Signal<ExternalEventSender>>()();
	send_event(&tx, event).await
}

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
}

impl ExternalEvent {
	pub fn handle(self) {
		match self {
			Self::FileListReset => {
				let mut fl_sig = use_context::<Signal<FileList>>();
				fl_sig.set(FileList::None);
			}
			Self::NonHashedFileListSet(new_fl) => {
				let mut fl_sig = use_context::<Signal<FileList>>();
				fl_sig.set(FileList::NonHashed(new_fl));
			}
			Self::LoadingBarAdd => {
				let mut lb_sig = use_context::<Signal<LoadingBarStatus>>();
				lb_sig.set(LoadingBarStatus::Displayed);
			}
			Self::LoadingBarDelete => {
				let mut lb_sig = use_context::<Signal<LoadingBarStatus>>();
				lb_sig.set(LoadingBarStatus::Hidden);
			}
			Self::ProgressBarAdd(nb) => {
				let mut pg_sig = use_context::<Signal<Option<ProgressBarStatus>>>();
				match pg_sig() {
					Some(mut status) => {
						status.add_progress(nb);
						pg_sig.set(Some(status));
					}
					None => {
						error!("No active progress bar for ProgressBarAdd({nb})");
					}
				}
			}
			Self::ProgressBarCreate(nb) => {
				let mut pg_sig = use_context::<Signal<Option<ProgressBarStatus>>>();
				pg_sig.set(Some(ProgressBarStatus::new(nb)));
			}
			Self::ProgressBarDelete => {
				let mut pg_sig = use_context::<Signal<Option<ProgressBarStatus>>>();
				pg_sig.set(None);
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
