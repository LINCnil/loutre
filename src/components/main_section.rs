#![allow(non_snake_case)]

use crate::app::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MainSectionProps {
	#[props(default = None)]
	close_view: Option<Route>,
	children: Element,
}

#[component]
pub fn MainSection(props: MainSectionProps) -> Element {
	rsx! {
		div {
			class: "component-main-section",
			div {
				class: "component-main-section-top",
				if let Some(target) = props.close_view {
					Link {
						to: target,
						i {
							class: "ri-close-circle-fill",
						}
					}
				}
			}
			div {
				class: "component-main-section-content",
				{ props.children }
			}
		}
	}
}
