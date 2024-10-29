use tokio::sync::mpsc::{Receiver, Sender};

pub type ExternalEventReceiver = Receiver<ExternalEvent>;
pub type ExternalEventSender = Sender<ExternalEvent>;

#[derive(Clone, Copy, Debug)]
pub enum ExternalEvent {
	ProgressBarAdd(usize),
	ProgressBarCreate(usize),
	ProgressBarDelete,
	ProgressBarSet((usize, usize)),
}
