#![allow(non_snake_case)]

use crate::progress::{LoadingBarStatus, ProgressBarStatus};
use dioxus::prelude::*;
use dioxus_i18n::tid;
use humansize::{make_format, DECIMAL};

#[component]
pub fn ProgressBar() -> Element {
	let status_opt = use_context::<Signal<Option<ProgressBarStatus>>>()();
	let formatter = make_format(DECIMAL);
	rsx! {
		if let Some(status) = status_opt {
			div {
				class: "component-progress-bar",
				progress {
					max: "{status.get_max()}",
					value: "{status.get_value()}",
				}
				p {{
					tid!(
						"cpn_progress_bar_status",
						done: formatter(status.get_value()),
						total: formatter(status.get_max()),
						percent: status.get_value() * 100 / status.get_max()
					)
				}}
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
