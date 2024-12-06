#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MainSectionProps {
	children: Element,
}

#[component]
pub fn MainSection(props: MainSectionProps) -> Element {
	rsx! {
		div {
			class: "component-main-section",
			{ props.children }
		}
	}
}
