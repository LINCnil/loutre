#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
	let logo = include_str!("../../assets/logo.svg");
	let logo = match logo.find("<svg") {
		Some(i) => logo.split_at(i).1,
		None => logo,
	};

	#[cfg(feature = "nightly")]
	let logo = logo
		.replace("#001d96", "#2242ff")
		.replace("#bbe4ff", "#2242ff")
		.replace("#6045ff", "#a95fff")
		.replace("#45efce", "#a95fff")
		.replace("#00000000", "#e3cfffff")
		.replace("#ffffff00", "#a95fffff");
	rsx! {
		div {
			dangerous_inner_html: "{logo}"
		}
	}
}
