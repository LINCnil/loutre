#![allow(non_snake_case)]

use crate::config::Config;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

async fn change_lang(config: &mut Config, to_lang: LanguageIdentifier) {
	// Change the current language
	let mut i18n = i18n();
	i18n.set_language(to_lang.clone());

	// Write it to the configuration
	config.lang = to_lang.into();
	config.write_to_file();
}

#[component]
pub fn LangSwitch() -> Element {
	let config = use_context::<Signal<Config>>();
	let langs = [
		(langid!("en-US"), "English (US)"),
		(langid!("fr-BE"), "Français (Belgique)"),
		(langid!("fr-FR"), "Français (France)"),
	];
	rsx! {
		div {
			class: "component-lang-switch",
			i {
				class: "ri-translate-2"
			}
			ul {
				class: "component-lang-switch-list",
				for (id, name) in langs {
					li {
						onclick: move |_| {
							let value = id.clone();
							spawn(async move {
								change_lang(&mut config(), value).await;
							});
						},
						"{name}"
					}
				}
			}
		}
	}
}
