#![allow(non_snake_case)]

use crate::app::Route::{Main, MainConfig};
use crate::components::{LangSwitch, Logo, ThemeSwitch};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(PartialEq, Clone, Props)]
pub struct HeaderProps {
	#[props(default = false)]
	is_config_view: bool,
	#[props(default = false)]
	is_debug_view: bool,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
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
					DebugLink {
						is_debug_view: props.is_debug_view,
					}
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
								to: if props.is_config_view {
									Main {}
								} else {
									MainConfig {}
								},
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
fn DebugLink(is_debug_view: bool) -> Element {
	rsx! {}
}

#[cfg(feature = "nightly")]
#[component]
fn DebugLink(is_debug_view: bool) -> Element {
	rsx! {
		li {
			span {
				class: "component-header-menu-item",
				Link {
					class: "ri-bug-line",
					to: if is_debug_view {
						Main {}
					} else {
						crate::app::Route::Debug {}
					},
					title: t!("cpn_header_config"),
				}
			}
		}
	}
}
