#![allow(non_snake_case)]

use crate::clipboard::{Clipboard, ClipboardStart};
use crate::components::HeaderLangSwitchDisplay;
use crate::config::Config;
use crate::events::{ExternalEventReceiver, ExternalEventSender, ExternalEventSignals};
use crate::files::FileList;
use crate::notifications::NotificationBlackList;
use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use crate::receipt::Receipt;
use crate::theme::{get_default_theme, set_theme, Theme};
use crate::views::*;
use dioxus::document::Style;
use dioxus::prelude::*;
use futures_util::StreamExt;

pub const STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/loutre.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
	#[route("/")]
	Main {},
	#[route("/check_errors")]
	CheckErrors {},
	#[route("/empty_files")]
	EmptyFiles {},
	#[route("/excluded_files")]
	ExcludedFiles {},
	#[route("/duplicated_files")]
	DuplicatedFiles {},
	#[route("/config/files")]
	FilesConfig {},
	#[route("/config/hash")]
	HashConfig {},
	#[route("/config/messages")]
	MessagesConfig {},
	#[route("/config/clipboard")]
	ClipboardConfig {},
	#[route("/config/clipboard/template/:tpl_id")]
	ClipboardTemplateConfig { tpl_id: usize },
	#[cfg(feature = "nightly")]
	#[route("/debug")]
	Debug {},
}

#[component]
pub fn App() -> Element {
	let config = Config::init();
	crate::i18n::init(&config);

	let handle = use_coroutine(|mut evt_rx: ExternalEventReceiver| async move {
		let mut signals = ExternalEventSignals::new();
		tracing::info!("Waiting for an external event…");
		while let Some(event) = evt_rx.next().await {
			tracing::info!("External event received: {event}");
			event.handle(&mut signals);
		}
	});

	initialize_global_context(config, handle.tx());
	initialize_theme();

	rsx! {
		Style {{ STYLE }}
		Router::<Route> {}
	}
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
						tracing::info!("loading theme from configuration: {t}");
						t
					}
					None => {
						let t = get_default_theme().await;
						tracing::info!("no theme configured, loading default one: {t}");
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
	use_context_provider(|| Signal::new(ClipboardStart::default()));

	// Configuration
	use_context_provider(|| Signal::new(config));

	// Theme
	use_context_provider(|| Signal::new(Theme::default()));

	// Files
	use_context_provider(|| Signal::new(FileList::default()));
	let receipt_status: Option<Receipt> = None;
	use_context_provider(|| Signal::new(receipt_status));

	// Header
	use_context_provider(|| Signal::new(HeaderLangSwitchDisplay::new()));

	// Notification blacklist
	use_context_provider(|| Signal::new(NotificationBlackList::new()));

	// Progress bar
	let pg_status: Option<ProgressBarStatus> = None;
	use_context_provider(|| Signal::new(pg_status));
	use_context_provider(|| Signal::new(progress_tx));
	use_context_provider(|| Signal::new(LoadingBarStatus::Hidden));
}
