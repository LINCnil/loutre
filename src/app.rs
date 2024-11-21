#![allow(non_snake_case)]

use crate::clipboard::Clipboard;
use crate::config::Config;
use crate::events::{ExternalEventReceiver, ExternalEventSender};
use crate::files::FileList;
use crate::notifications::NotificationBlackList;
use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use crate::receipt::Receipt;
use crate::theme::{get_default_theme, set_theme, Theme};
use crate::views::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use tokio::sync::mpsc::channel;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
	#[route("/")]
	Main {},
	#[route("/config/main")]
	MainConfig {},
	#[route("/config/hash")]
	HashConfig {},
	#[route("/config/messages")]
	MessagesConfig {},
	#[route("/config/clipboard")]
	ClipboardConfig {},
	#[cfg(feature = "nightly")]
	#[route("/debug")]
	Debug {},
}

#[component]
pub fn App() -> Element {
	let config = Config::init();
	let (progress_tx, progress_rx) = channel(crate::PROGRESS_BAR_CHANNEL_CAPACITY);

	crate::i18n::init(&config);
	initialize_global_context(config, progress_tx);
	initialize_theme();
	listen_to_progress_bar_changes(progress_rx);

	rsx! {
		Router::<Route> {}
	}
}

fn listen_to_progress_bar_changes(mut progress_rx: ExternalEventReceiver) -> Coroutine<()> {
	let cfg_sig = use_context::<Signal<Config>>();
	let clb_sig = use_context::<Signal<Clipboard>>();
	let fl_sig = use_context::<Signal<FileList>>();
	let lb_sig = use_context::<Signal<LoadingBarStatus>>();
	let pg_sig = use_context::<Signal<Option<ProgressBarStatus>>>();
	let rcpt_sig = use_context::<Signal<Option<Receipt>>>();
	use_coroutine(|_| async move {
		info!("Waiting for an external event…");
		while let Some(event) = progress_rx.recv().await {
			info!("External event received: {event}");
			event.handle(cfg_sig, clb_sig, fl_sig, lb_sig, pg_sig, rcpt_sig);
		}
	})
}

fn initialize_theme() {
	let config_sig = use_context::<Signal<Config>>();
	let theme_sig = use_context::<Signal<Theme>>();
	let mut is_init = true;
	use_effect(move || {
		if is_init {
			is_init = false;
			let config = config_sig();
			spawn(async move {
				let default_theme = match config.theme {
					Some(t) => {
						info!("loading theme from configuration: {t}");
						t
					}
					None => {
						let t = get_default_theme().await;
						info!("no theme configured, loading default one: {t}");
						t
					}
				};
				set_theme(config_sig, theme_sig, default_theme).await;
			});
		}
	});
}

fn initialize_global_context(config: Config, progress_tx: ExternalEventSender) {
	// Clipboard
	use_context_provider(|| Signal::new(Clipboard::new()));

	// Configuration
	use_context_provider(|| Signal::new(config));

	// Theme
	use_context_provider(|| Signal::new(Theme::default()));

	// Files
	use_context_provider(|| Signal::new(FileList::default()));
	let receipt_status: Option<Receipt> = None;
	use_context_provider(|| Signal::new(receipt_status));

	// Notification blacklist
	use_context_provider(|| Signal::new(NotificationBlackList::new()));

	// Progress bar
	let pg_status: Option<ProgressBarStatus> = None;
	use_context_provider(|| Signal::new(pg_status));
	use_context_provider(|| Signal::new(progress_tx));
	use_context_provider(|| Signal::new(LoadingBarStatus::Hidden));
}
