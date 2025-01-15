#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ConfigElementProps {
	id: String,
	label: String,
	children: Element,
}

#[component]
pub fn ConfigElement(props: ConfigElementProps) -> Element {
	rsx! {
		div {
			class: "component-config-element",
			p {
				label {
					r#for: props.id,
					"{props.label}"
				}
			}
			div {
				{ props.children }
			}
		}
	}
}
