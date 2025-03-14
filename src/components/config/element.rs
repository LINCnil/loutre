#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ConfigElementProps {
	id: String,
	label: String,
	#[props(default = None)]
	tooltip: Option<String>,
	children: Element,
}

#[component]
pub fn ConfigElement(props: ConfigElementProps) -> Element {
	rsx! {
		div {
			class: "component-config-element",
			p {
				if let Some(msg) = props.tooltip {
					label {
						r#for: props.id,
						title: "{msg}",
						"{props.label}"
						i {
							class: "ri-information-2-line tooltip-icon"
						}
					}
				} else {
					label {
						r#for: props.id,
						"{props.label}"
					}
				}
			}
			div {
				{ props.children }
			}
		}
	}
}
