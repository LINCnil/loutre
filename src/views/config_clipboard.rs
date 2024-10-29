#![allow(non_snake_case)]

use crate::components::{ConfigMenu, ConfigMenuHighlight, DropZone, Header};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ClipboardConfig() -> Element {
	rsx! {
		DropZone {
			Header {
				is_config_view: true,
				h1 {
					{ t!("view_config_title") }
				}
			}
			ConfigMenu { hl: ConfigMenuHighlight::Clipboard }
			p { "Debug: ClipboardConfigView" }
		}
	}
}
