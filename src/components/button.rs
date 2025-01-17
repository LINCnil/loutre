#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
	#[props(default = String::new())]
	icon: String,
	onclick: EventHandler<MouseEvent>,
	children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	rsx! {
		div {
			class: "component-button-wrapper",
			button {
				class: "component-button",
				onclick: move |evt| props.onclick.call(evt),
				if !props.icon.is_empty() {
					div {
						class: "component-button-icon",
						span {
							class: "{props.icon}",
						}
					}
					div {
						class: "component-button-sep",
					}
				}
				div {
					class: "component-button-text",
					{props.children}
				}
			}
		}
	}
}

#[derive(PartialEq, Clone, Props)]
pub struct FileButtonProps {
	icon: String,
	accept: String,
	multiple: bool,
	directory: bool,
	name: String,
	onchange: EventHandler<FormEvent>,
	children: Element,
}

#[component]
pub fn FileButton(props: FileButtonProps) -> Element {
	let id = format!("components-button-file-id-{}", props.name);
	rsx! {
		label {
			class: "component-button-file-label",
			r#for: "{id}",
			span {
				class: "{props.icon} component-button-icon",
			}
			" "
			{props.children}
		}
		input {
			id: "{id}",
			name: "{props.name}",
			r#type: "file",
			class: "component-button-file-input",
			accept: "{props.accept}",
			multiple: props.multiple,
			directory: props.directory,
			onchange: move |evt| props.onchange.call(evt),
		}
	}
}
