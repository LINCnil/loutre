#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
	id: String,
	name: String,
	#[props(default = false)]
	checked: bool,
	onchange: EventHandler<FormEvent>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
	rsx! {
		input {
			id: props.id,
			name: props.name,
			class: "component-form-checkbox",
			r#type: "checkbox",
			checked: props.checked,
			onclick: move |evt| evt.prevent_default(),
			onchange: move |evt| props.onchange.call(evt),
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct SelectOption {
	name: String,
	value: String,
}

impl SelectOption {
	pub fn new<S: AsRef<str>>(name: S, value: S) -> Self {
		Self {
			name: name.as_ref().to_string(),
			value: value.as_ref().to_string(),
		}
	}
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
	id: String,
	name: String,
	options: Vec<SelectOption>,
	selected_option: String,
	onchange: EventHandler<FormEvent>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
	rsx! {
		select {
			id: props.id,
			name: props.name,
			class: "component-form-select",
			onchange: move |evt| {
				evt.prevent_default();
				props.onchange.call(evt)
			},
			for opt in props.options {
				option {
					value: "{opt.value}",
					selected: opt.value == props.selected_option,
					"{opt.name}"
				}
			}
		}
	}
}
