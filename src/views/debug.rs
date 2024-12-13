#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::{DropZone, Header, LoadingBar, MainSection, Notification, ProgressBar};
use crate::events::{send_event, ExternalEvent, ExternalEventSender};
use crate::notifications::NotificationLevel;
use crate::progress::LoadingBarStatus;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

const LOREM_LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

macro_rules! form_value_str {
	($data: ident, $name: literal) => {
		$data.get($name).unwrap().as_value().as_str()
	};
}

#[component]
pub fn Debug() -> Element {
	let mut notifs = use_signal(Vec::<(NotificationLevel, usize)>::new);
	let loading_bar = use_context::<Signal<LoadingBarStatus>>()();
	let tx_sig = use_context::<Signal<ExternalEventSender>>();

	rsx! {
		DropZone {
			Header {
				is_debug_view: true,
			}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 { "Debug" }
				form {
					onsubmit: move |event| {
						info!("Notification form event: {event:?}");
						let data = event.data.values();
						info!("Notification form event data: {data:?}");
						let level: NotificationLevel = form_value_str!(data, "notif_level").parse().unwrap();
						let nb: usize = form_value_str!(data, "nb").parse().unwrap();
						notifs.push((level, nb));
					},
					fieldset {
						legend { "Notifications" }
						select {
							name: "notif_level",
							option { "error" }
							option { "warning" }
							option { "success" }
							option { "info" }
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
						let nb: u64 = form_value_str!(data, "nb").parse().unwrap();
						let tx = tx_sig();
						spawn(async move {
							send_event(&tx, ExternalEvent::ProgressBarDelete);
							send_event(&tx, ExternalEvent::ProgressBarCreate(100));
							send_event(&tx, ExternalEvent::ProgressBarAdd(nb));
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
							onclick: move |evt| {
								evt.prevent_default();
								info!("Debug: Progress bar button onclick");
								let tx = tx_sig();
								spawn(async move {
									send_event(&tx, ExternalEvent::ProgressBarDelete);
								});
							},
							"Reset"
						}
					}
				}

				fieldset {
					legend { "Loading bar" }
					button {
						onclick: move |evt| {
							evt.prevent_default();
							info!("Debug: Loading bar button onclick");
							let tx = tx_sig();
							spawn(async move {
								let new_status_evt = match loading_bar {
									LoadingBarStatus::Displayed => ExternalEvent::LoadingBarDelete,
									LoadingBarStatus::Hidden => ExternalEvent::LoadingBarAdd,
								};
								send_event(&tx, new_status_evt);
							});
						},
						"Toogle"
					}
				}

				ProgressBar {}
				LoadingBar {}

				for (i, (level, nb)) in notifs().iter().enumerate() {
					Notification {
						id: "debug_{i}",
						level: *level,
						title: "Debug notification ({level})",
						for _ in 0..*nb {
							p { "{LOREM_LIPSUM}" }
						}
					}
				}
			}
		}
	}
}
