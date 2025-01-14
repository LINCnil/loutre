#![allow(non_snake_case)]

use crate::components::Button;
use dioxus::prelude::*;
use dioxus_i18n::t;
use tokio::time::{sleep, Duration};

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
		div {
			label {
				class: "component-form-checkbox",
				input {
					id: props.id,
					name: props.name,
					r#type: "checkbox",
					checked: props.checked,
					onchange: move |evt| props.onchange.call(evt),
				}
				span {
					class: "component-form-checkbox-slider",
				}
			}
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

#[derive(PartialEq, Clone, Props)]
pub struct ApplyConfigProps {
	onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn ApplyConfig(props: ApplyConfigProps) -> Element {
	let mut display_msg = use_signal(|| false);

	rsx! {
		div {
			class: "component-form-apply-config",
			Button {
				onclick: move |evt| async move {
					props.onclick.call(evt);
					display_msg.set(false);
					sleep(Duration::from_millis(100)).await;
					display_msg.set(true);
				},
				{ t!("cpn_form_apply_config") }
			}
			if display_msg() {
				p {
					class: "component-form-apply-config-msg component-form-apply-config-fade-out",
					span {
						class: "ri-check-line",
					}
					{ t!("cpn_form_apply_config_ok") }
				}
			}
		}
	}
}
