#![allow(non_snake_case)]

use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HeaderLangSwitchDisplay(bool);

impl HeaderLangSwitchDisplay {
	pub fn new() -> Self {
		HeaderLangSwitchDisplay(false)
	}

	pub fn toggle(&mut self) {
		self.0 = !self.0;
	}

	pub fn get_cls(self) -> String {
		let mut s = String::from("component-lang-switch-list");
		if self.0 {
			s += " component-lang-switch-list-display";
		} else {
			s += " component-lang-switch-list-hidden";
		}
		s
	}
}

async fn change_lang(config: &mut Config, to_lang: LanguageIdentifier) {
	// Change the current language
	let mut i18n = i18n();
	i18n.set_language(to_lang.clone());

	// Write it to the configuration
	config.lang = to_lang.into();
	config.write_to_file();
}

fn toggle_display() {
	let mut hls_sig = use_context::<Signal<HeaderLangSwitchDisplay>>();
	let mut hls = hls_sig();
	hls.toggle();
	hls_sig.set(hls);
}

#[component]
pub fn LangSwitch() -> Element {
	let config_sig = use_context::<Signal<Config>>();
	let langs = [
		(langid!("en-US"), "English (US)"),
		(langid!("fr-BE"), "Français (Belgique)"),
		(langid!("fr-FR"), "Français (France)"),
	];
	let lst_cls = use_context::<Signal<HeaderLangSwitchDisplay>>()().get_cls();

	rsx! {
		div {
			class: "component-lang-switch",
			span {
				class: "component-header-menu-item",
				i {
					class: "ri-translate-2",
					onclick: move |_| {
						toggle_display();
					},
				}
			}
			ul {
				class: "{lst_cls}",
				for (id, name) in langs {
					li {
						onclick: move |_| {
							let value = id.clone();
							spawn(async move {
								change_lang(&mut config_sig(), value).await;
								toggle_display();
							});
						},
						"{name}"
					}
				}
			}
		}
	}
}
