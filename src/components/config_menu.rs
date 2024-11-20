#![allow(non_snake_case)]

use crate::app::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(PartialEq, Clone, Copy)]
pub enum ConfigMenuHighlight {
	Main,
	Messages,
	Clipboard,
}

#[component]
pub fn ConfigMenu(hl: ConfigMenuHighlight) -> Element {
	rsx! {
		ul {
			class: "component-config-menu",
			ConfigMenuElement {
				target: Route::MainConfig {},
				target_str: "cpn_config_menu_main_title",
				hl: hl,
				current: ConfigMenuHighlight::Main
			}
			ConfigMenuElement {
				target: Route::MessagesConfig {},
				target_str: "cpn_config_menu_messages_title",
				hl: hl,
				current: ConfigMenuHighlight::Messages
			}
			ConfigMenuElement {
				target: Route::ClipboardConfig {},
				target_str: "cpn_config_menu_clipboard_title",
				hl: hl,
				current: ConfigMenuHighlight::Clipboard
			}
		}
	}
}

fn get_class(hl: ConfigMenuHighlight, current: ConfigMenuHighlight) -> &'static str {
	if hl == current {
		"component-config-menu-elem component-config-menu-elem-hl"
	} else {
		"component-config-menu-elem"
	}
}

#[component]
fn ConfigMenuElement(
	target: Route,
	target_str: &'static str,
	hl: ConfigMenuHighlight,
	current: ConfigMenuHighlight,
) -> Element {
	rsx! {
		li {
			class: get_class(hl, current),
			Link {
				to: target,
				{ t!(target_str) }
			}
		}
	}
}
