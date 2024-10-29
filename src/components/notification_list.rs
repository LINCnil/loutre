#![allow(non_snake_case)]

use crate::notifications::{Notification, NotificationLevel, NotificationList};
use dioxus::prelude::*;
use uuid::Uuid;

async fn close_notification(id: Uuid) {
	let mut notif_lst_sig = use_context::<Signal<NotificationList>>();
	let mut notif_lst = notif_lst_sig();
	notif_lst.remove(&id);
	notif_lst_sig.set(notif_lst);
}

#[component]
pub fn NotificationList() -> Element {
	let notif_lst = use_context::<Signal<NotificationList>>()();
	let mut notif_lst = notif_lst.to_vec();
	notif_lst.sort();

	rsx! {
		for notif in notif_lst {
			NotificationElement {
				notif: notif.clone()
			}
		}
	}
}

#[component]
pub fn NotificationElement(notif: Notification) -> Element {
	rsx! {
		div {
			class: "component-notification component-notification-level-{notif.get_level()}",
			div {
				class: "component-notification-icon",
				span {
					class: match notif.get_level() {
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
				h3 {
					"{notif.get_title()}"
				}
				if notif.is_html() {
					div {
						dangerous_inner_html: "{notif}",
					}
				} else {
					p {
						"{notif}"
					}
				}
			}
			div {
				class: "component-notification-close",
				span {
					class: "ri-close-large-line",
					onclick: move |_| {
						let id = notif.get_id();
						spawn(async move {
							close_notification(id).await;
						});
					},
				}
			}
		}
	}
}
