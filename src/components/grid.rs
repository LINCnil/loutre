#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GridProps {
	children: Element,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
	rsx! {
		div {
			class: "component-grid",
			{ props.children }
		}
	}
}
