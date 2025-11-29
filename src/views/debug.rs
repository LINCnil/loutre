#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::{Button, Header, LoadingBar, MainSection, Notification, ProgressBar, Root};
use crate::events::{send_event, ExternalEvent, ExternalEventSender};
use crate::notifications::NotificationLevel;
use crate::progress::LoadingBarStatus;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use serde::{Deserialize, Serialize};

const LOREM_LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

#[derive(Deserialize, Serialize)]
pub struct NotificationForm {
	level: String,
	nb: usize,
}

impl NotificationForm {
	fn get_level(&self) -> NotificationLevel {
		self.level.parse().unwrap()
	}
}

#[derive(Deserialize, Serialize)]
pub struct ProgressBarForm {
	nb: u64,
}

#[component]
pub fn Debug() -> Element {
	let mut notifs = use_signal(Vec::<(NotificationLevel, usize)>::new);
	let loading_bar = use_context::<Signal<LoadingBarStatus>>()();
	let tx_sig = use_context::<Signal<ExternalEventSender>>();

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ tid!("view_debug_title") }
				}
				form {
					onsubmit: move |event| {
						tracing::info!("Notification form event: {event:?}");
						let data: NotificationForm = event.parsed_values().unwrap();
						//let data = event.data.values();
						//tracing::info!("Notification form event data: {data:?}");
						//let level: NotificationLevel = form_value_str!(data, "notif_level").parse().unwrap();
						//let nb: usize = form_value_str!(data, "nb").parse().unwrap();
						notifs.push((data.get_level(), data.nb));
					},
					fieldset {
						legend {
							{ tid!("view_debug_notif_title") }
						}
						select {
							name: "notif_level",
							option {
								value: "error",
								{ tid!("view_debug_notif_level_error") }
							}
							option {
								value: "warning",
								{ tid!("view_debug_notif_level_warning") }
							}
							option {
								value: "success",
								{ tid!("view_debug_notif_level_success") }
							}
							option {
								value: "info",
								{ tid!("view_debug_notif_level_info") }
							}
						}
						input {
							name: "nb",
							r#type: "number",
							value: 1,
							min: 1,
						}
						input {
							class: "component-form-fieldset-right",
							r#type: "submit",
							value: tid!("view_debug_submit"),
						}
					}
				}

				form {
					onsubmit: move |event| {
						tracing::info!("Progress bar form event: {event:?}");
						let data: ProgressBarForm = event.parsed_values().unwrap();
						//let data = event.data.values();
						//tracing::info!("Progress bar form event data: {data:?}");
						//let nb: u64 = form_value_str!(data, "nb").parse().unwrap();
						let tx = tx_sig();
						spawn(async move {
							send_event(&tx, ExternalEvent::ProgressBarDelete);
							send_event(&tx, ExternalEvent::ProgressBarCreate(100));
							send_event(&tx, ExternalEvent::ProgressBarAdd(data.nb));
						});
					},
					fieldset {
						legend {
							{ tid!("view_debug_progress_bar_title") }
						}
						input {
							name: "nb",
							r#type: "number",
							value: 42,
							min: 0,
						}
						input {
							class: "component-form-fieldset-right",
							r#type: "submit",
							value: tid!("view_debug_submit"),
						}
						Button {
							onclick: move |evt: MouseEvent| {
								evt.prevent_default();
								tracing::info!("Debug: Progress bar button onclick");
								let tx = tx_sig();
								spawn(async move {
									send_event(&tx, ExternalEvent::ProgressBarDelete);
								});
							},
							{ tid!("view_debug_reset") }
						}
					}
				}

				fieldset {
					legend {
						{ tid!("view_debug_loading_bar_title") }
					}
					Button {
						onclick: move |evt: MouseEvent| {
							evt.prevent_default();
							tracing::info!("Debug: Loading bar button onclick");
							let tx = tx_sig();
							spawn(async move {
								let new_status_evt = match loading_bar {
									LoadingBarStatus::Displayed => ExternalEvent::LoadingBarDelete,
									LoadingBarStatus::Hidden => ExternalEvent::LoadingBarAdd,
								};
								send_event(&tx, new_status_evt);
							});
						},
						{ tid!("view_debug_toogle") }
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
