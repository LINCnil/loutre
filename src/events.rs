use crate::files::NonHashedFileList;
use tokio::sync::mpsc::{Receiver, Sender};

pub type ExternalEventReceiver = Receiver<ExternalEvent>;
pub type ExternalEventSender = Sender<ExternalEvent>;

#[derive(Clone, Debug)]
pub enum ExternalEvent {
	NonHashedFileListSet(NonHashedFileList),
	LoadingBarAdd,
	LoadingBarDelete,
	ProgressBarAdd(usize),
	ProgressBarCreate(usize),
	ProgressBarDelete,
	ProgressBarSet((usize, usize)),
}
