#![allow(non_snake_case)]

use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use dioxus::prelude::*;

#[component]
pub fn ProgressBar() -> Element {
	let status_opt = use_context::<Signal<Option<ProgressBarStatus>>>()();
	rsx! {
		if let Some(status) = status_opt {
			div {
				class: "component-progress-bar",
				progress {
					max: "{status.get_max()}",
					value: "{status.get_value()}",
				}
			}
		}
	}
}

#[component]
pub fn LoadingBar() -> Element {
	let loading_bar = use_context::<Signal<LoadingBarStatus>>()();
	rsx! {
		if loading_bar.is_displayed() {
			div {
				class: "component-progress-bar",
				progress {}
			}
		}
	}
}
