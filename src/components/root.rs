#![allow(non_snake_case)]

use dioxus::prelude::*;

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
