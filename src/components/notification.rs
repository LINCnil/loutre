#![allow(non_snake_case)]

use crate::notifications::{NotificationBlackList, NotificationLevel};
use dioxus::prelude::*;

#[component]
pub fn RawNotificationList() -> Element {
	rsx! {}
}

#[component]
pub fn NotificationList() -> Element {
	rsx! {}
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
