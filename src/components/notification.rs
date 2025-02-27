#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::Button;
use crate::config::Config;
use crate::files::FileList;
use crate::notifications::{NotificationBlackList, NotificationLevel};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn NotificationList() -> Element {
	let fl = use_context::<Signal<FileList>>()();
	let cfg = use_context::<Signal<Config>>()();
	let nb_empty_files = fl.nb_empty_files();
	let nb_excluded_files = fl.nb_excluded_files();

	rsx! {
		if cfg.is_empty_file_warning_enabled() && nb_empty_files != 0 {
			Notification {
				id: "empty_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: tid!("cpn_notif_empty_files_title", nb: nb_empty_files),
				p { { tid!("cpn_notif_empty_files_text", nb: nb_empty_files) } }
				p {
					Button {
						onclick: move |_event| {
							navigator().push(Route::EmptyFiles {});
						},
						{ tid!("cpn_notif_empty_files_link", nb: nb_empty_files) }
					}
				}
			}
		}

		if nb_excluded_files != 0 {
			Notification {
				id: "excluded_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: tid!("cpn_notif_excluded_files_title", nb: nb_excluded_files),
				p { { tid!("cpn_notif_excluded_files_text", nb: nb_excluded_files) } }
				p {
					Button {
						onclick: move |_event| {
							navigator().push(Route::ExcludedFiles {});
						},
						{ tid!("cpn_notif_excluded_files_link", nb: nb_excluded_files) }
					}
				}
			}
		}

		if cfg.is_duplicate_file_warning_enabled() && fl.has_duplicated_files() {
			Notification {
				id: "duplicated_files_{fl.get_id()}",
				level: NotificationLevel::Warning,
				title: tid!("cpn_notif_duplicated_files_title"),
				p { { tid!("cpn_notif_duplicated_files_text") } }
				p {
					Button {
						onclick: move |_event| {
							navigator().push(Route::DuplicatedFiles {});
						},
						{ tid!("cpn_notif_duplicated_files_link", nb: nb_empty_files) }
					}
				}
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
							NotificationLevel::Error => { "ri-close-circle-fill" },
							NotificationLevel::Warning => { "ri-alert-fill" },
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
