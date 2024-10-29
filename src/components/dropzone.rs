#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DropZoneProps {
	#[props(default = EventHandler::default())]
	ondrop: EventHandler<DragEvent>,
	#[props(default = EventHandler::default())]
	ondragover: EventHandler<DragEvent>,
	children: Element,
}

#[component]
pub fn DropZone(props: DropZoneProps) -> Element {
	rsx! {
		div {
			class: "component-dropzone",
			id: "component-dropzone-main-area",
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
