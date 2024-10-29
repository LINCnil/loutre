#![allow(non_snake_case)]

use crate::components::{DropZone, Header, LoadingBar, NotificationList, ProgressBar};
use crate::events::{ExternalEvent, ExternalEventSender};
use crate::notifications::*;
use crate::progress::LoadingBarStatus;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};

const LOREM_LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

macro_rules! form_value_str {
	($data: ident, $name: literal) => {
		$data.get($name).unwrap().as_value().as_str()
	};
}

#[component]
pub fn Debug() -> Element {
	let mut notif_lst_sig = use_context::<Signal<NotificationList>>();

	rsx! {
		DropZone {
			Header {
				is_debug_view: true,
				h1 { "Debug" }
			}

			form {
				onsubmit: move |event| {
					info!("Notification form event: {event:?}");
					let data = event.data.values();
					info!("Notification form event data: {data:?}");
					let mut notif_lst = notif_lst_sig();
					let nb: usize = form_value_str!(data, "nb").parse().unwrap();
					let mut txt = String::new();
					for _ in 0..nb {
						txt += &format!("<p>{LOREM_LIPSUM}</p>");
					}
					let mut notif = Notification::new(
						get_notification_level(form_value_str!(data, "notif_type")),
						get_notification_context(form_value_str!(data, "notif_ctx")),
						format!("Debug notification (context: {})", form_value_str!(data, "notif_ctx")),
						"",
					);
					notif.set_html(txt);
					notif_lst.insert(notif);
					notif_lst_sig.set(notif_lst);
				},
				fieldset {
					legend { "Notifications" }
					select {
						name: "notif_type",
						option { "error" }
						option { "warning" }
						option { "success" }
						option { "info" }
					}
					select {
						name: "notif_ctx",
						option { "file list" }
						option { "receipt" }
						option { "computed hash" }
					}
					input {
						name: "nb",
						r#type: "number",
						value: 1,
						min: 1,
					}
					input { r#type: "submit" }
				}
			}

			form {
				onsubmit: move |event| {
					info!("Progress bar form event: {event:?}");
					let data = event.data.values();
					info!("Progress bar form event data: {data:?}");
					let nb: usize = form_value_str!(data, "nb").parse().unwrap();
					spawn(async move {
						let pg_tx = use_context::<Signal<ExternalEventSender>>()();
						if let Err(e) = pg_tx.send(ExternalEvent::ProgressBarSet((100, nb))).await {
							error!("Error sending progress bar message: {e}");
						}
					});
				},
				fieldset {
					legend { "Progress bar" }
					input {
						name: "nb",
						r#type: "number",
						value: 42,
						min: 0,
					}
					input { r#type: "submit" }
					button {
						prevent_default: "onclick",
						onclick: |_event| {
							info!("Debug: Progress bar button onclick");
							spawn(async move {
								let pg_tx = use_context::<Signal<ExternalEventSender>>()();
								if let Err(e) = pg_tx.send(ExternalEvent::ProgressBarDelete).await {
									error!("Error sending progress bar message: {e}");
								}
							});
						},
						"Reset"
					}
				}
			}

			fieldset {
				legend { "Loading bar" }
				button {
					prevent_default: "onclick",
					onclick: |_event| {
						info!("Debug: Loading bar button onclick");
						spawn(async move {
							let mut loading_sig = use_context::<Signal<LoadingBarStatus>>();
							let new_status = match loading_sig() {
								LoadingBarStatus::Displayed => LoadingBarStatus::Hidden,
								LoadingBarStatus::Hidden => LoadingBarStatus::Displayed,
							};
							loading_sig.set(new_status);
						});
					},
					"Toogle"
				}
			}

			NotificationList {}
			ProgressBar {}
			LoadingBar {}
		}
	}
}

fn get_notification_level(s: &str) -> NotificationLevel {
	match s.to_lowercase().as_str() {
		"error" => NotificationLevel::Error,
		"warning" => NotificationLevel::Warning,
		"success" => NotificationLevel::Success,
		"info" => NotificationLevel::Info,
		_ => NotificationLevel::Info,
	}
}

fn get_notification_context(s: &str) -> NotificationContext {
	match s.to_lowercase().as_str() {
		"file list" => NotificationContext::FileList,
		"receipt" => NotificationContext::Receipt,
		"computed hash" => NotificationContext::ComputedHash,
		_ => NotificationContext::FileList,
	}
}
