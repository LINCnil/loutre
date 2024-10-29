#![allow(non_snake_case)]

use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
	let theme = use_context::<Signal<Theme>>()();
	let img_b64 = match theme {
		Theme::Dark => crate::assets::LOGO_DARK_B64,
		Theme::Light => crate::assets::LOGO_LIGHT_B64,
	};
	rsx! {
		img {
			class: "component-logo",
			src: "{img_b64}",
		}
	}
}
