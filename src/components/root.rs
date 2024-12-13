#![allow(non_snake_case)]

use dioxus::document::Style;
use dioxus::prelude::*;

pub const STYLE: &str = include_str!(concat!(env!("OUT_DIR"), "/loutre.css"));

#[derive(PartialEq, Clone, Props)]
pub struct RootProps {
	#[props(default = EventHandler::default())]
	ondrop: EventHandler<DragEvent>,
	#[props(default = EventHandler::default())]
	ondragover: EventHandler<DragEvent>,
	children: Element,
}

#[component]
pub fn Root(props: RootProps) -> Element {
	rsx! {
		Style {{ STYLE }}
		div {
			class: "component-root",
			ondrop: move |event| {
				props.ondrop.call(event)
			},
			ondragover: move |event| {
				props.ondragover.call(event)
			},
			{ props.children }
		}
	}
}
