#![allow(non_snake_case)]

use crate::config::Config;
use crate::theme::{set_theme, Theme};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ThemeSwitch() -> Element {
	let config_sig = use_context::<Signal<Config>>();
	let theme_sig = use_context::<Signal<Theme>>();
	let theme = use_context::<Signal<Theme>>()();
	let (class, target) = match theme {
		Theme::Dark => ("ri-sun-line", Theme::Light),
		Theme::Light => ("ri-moon-line", Theme::Dark),
	};
	rsx! {
		span {
			class: "component-header-menu-item",
			i {
				class: "{class}",
				title: t!("cpn_theme_change"),
				onclick: move |_| {
					spawn(async move {
						set_theme(config_sig, theme_sig, target).await;
					});
				},
			}
		}
	}
}
