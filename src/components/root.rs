#![allow(non_snake_case)]

use dioxus::document::Style;
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
		Style {{ get_style() }}
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

fn get_style() -> String {
	let mut style = include_str!("../../assets/fonts/remixicon.css").to_string();
	style += include_str!("../../assets/loutre.css");
	style += &get_css_font(crate::assets::FONT_REMIXICON_B64, "remixicon", "woff2");
	style += &get_css_font(crate::assets::FONT_OPEN_SANS_B64, "Open Sans", "woff2");
	style
}

fn get_css_font(b64: &str, family: &str, format: &str) -> String {
	format!(
		"\n@font-face {{ font-family: '{}'; src: url({}) format('{}'); }}",
		family, b64, format
	)
}
