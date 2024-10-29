#![allow(non_snake_case)]

use crate::components::{Button, ConfigMenu, ConfigMenuHighlight, DropZone, Header};
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn MainConfig() -> Element {
	rsx! {
		DropZone {
			Header {
				is_config_view: true,
				h1 {
					{ t!("view_config_title") }
				}
			}
			ConfigMenu { hl: ConfigMenuHighlight::Main }
			p { "Debug: MainConfigView" }

			Button {
				onclick: move |_| println!("You clicked me!"),
				"My button"
			}
			Button {
				onclick: move |_| println!("You clicked me!"),
				icon: "ri-home-3-line",
				"My button with icon"
			}
			Button {
				onclick: move |_| println!("You clicked me!"),
				icon: "ri-home-3-line",
				p { "My button with icon" }
				p { "And multiple elements" }
				p { "yeah" }
			}
			Button {
				onclick: move |_| println!("You clicked me!"),
				icon: "ri-home-3-line",
			}
		}
	}
}
