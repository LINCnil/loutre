#![allow(non_snake_case)]

use crate::config::Config;
use crate::files::FileList;
use crate::notifications::{NotificationBlackList, NotificationLevel};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn NotificationList() -> Element {
	let fl = use_context::<Signal<FileList>>()();
	let cfg = use_context::<Signal<Config>>()();

	rsx! {
		if cfg.is_empty_file_warning_enabled() && fl.has_empty_files() {
			Notification {
				id: "empty_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: t!("cpn_notif_empty_files_title"),
				p { { t!("cpn_notif_empty_files_text") } }
			}
		}

		if fl.has_excluded_files() {
			Notification {
				id: "excluded_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: t!("cpn_notif_excluded_files_title"),
				p { { t!("cpn_notif_excluded_files_text") } }
			}
		}

		if cfg.is_duplicate_file_warning_enabled() && fl.has_duplicated_files() {
			Notification {
				id: "duplicated_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: t!("cpn_notif_duplicated_files_title"),
				p { { t!("cpn_notif_duplicated_files_text") } }
			}
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct NotificationProps {
	id: String,
	level: NotificationLevel,
	title: String,
	children: Element,
}

#[component]
pub fn Notification(props: NotificationProps) -> Element {
	let mut bl_sig = use_context::<Signal<NotificationBlackList>>();

	rsx! {
		if !bl_sig().contains(&props.id) {
			div {
				class: "component-notification component-notification-level-{props.level}",
				div {
					class: "component-notification-icon",
					span {
						class: match props.level {
							#[cfg(feature = "nightly")]
							NotificationLevel::Error => { "ri-close-circle-fill" },
							NotificationLevel::Warning => { "ri-alert-fill" },
							#[cfg(feature = "nightly")]
							NotificationLevel::Success => { "ri-checkbox-circle-fill" },
							NotificationLevel::Info => { "ri-information-2-fill" },
						},
					}
				}
				div {
					class: "component-notification-content",
					h3 { "{props.title}" }
					div {
						{ props.children }
					}
				}
				div {
					class: "component-notification-close",
					span {
						class: "ri-close-large-line",
						onclick: move |_| {
							let id = props.id.clone();
							spawn(async move {
								let mut bl = bl_sig();
								bl.insert(id);
								bl_sig.set(bl);
							});
						},
					}
				}
			}
		}
	}
}
