#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
	id: String,
	#[props(default = false)]
	checked: bool,
	onchange: EventHandler<FormEvent>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
	rsx! {
		input {
			id: props.id,
			class: "component-form-checkbox",
			r#type: "checkbox",
			checked: props.checked,
			prevent_default: "onchange",
			onchange: move |evt| props.onchange.call(evt),
		}
	}
}
