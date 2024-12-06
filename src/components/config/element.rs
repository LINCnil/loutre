#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ConfigElementProps {
	children: Element,
}

#[component]
pub fn ConfigElement(props: ConfigElementProps) -> Element {
	rsx! {
		div {
			class: "component-config-element",
			{ props.children }
		}
	}
}
