#![allow(non_snake_case)]

mod lang_switch;
mod logo;
mod theme_switch;

use self::lang_switch::LangSwitch;
use self::logo::Logo;
use self::theme_switch::ThemeSwitch;
use crate::app::Route::FilesConfig;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Header() -> Element {
	rsx! {
		header {
			class: "component-header",
			div {
				class: "component-header-logo",
				Logo {}
			}
			div {
				class: "component-header-right",
				menu {
					class: "component-header-menu",
					DebugLink {}
					li {
						LangSwitch {}
					}
					li {
						ThemeSwitch {}
					}
					li {
						span {
							class: "component-header-menu-item",
							Link {
								class: "ri-settings-3-line",
								to: FilesConfig {},
								title: t!("cpn_header_config"),
							}
						}
					}
				}
			}
		}
	}
}

#[cfg(not(feature = "nightly"))]
#[component]
fn DebugLink() -> Element {
	rsx! {}
}

#[cfg(feature = "nightly")]
#[component]
fn DebugLink() -> Element {
	rsx! {
		li {
			span {
				class: "component-header-menu-item",
				Link {
					class: "ri-bug-line",
					to: crate::app::Route::Debug {},
					title: t!("cpn_header_config"),
				}
			}
		}
	}
}
