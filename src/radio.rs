use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::utils::StaticCallback;

#[wasm_bindgen(module = "/build/mwc-radio.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Radio;

    #[wasm_bindgen(getter, static_method_of = Radio)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Radio) -> bool;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Radio, value: bool);
}

loader_hack!(Radio);

/// Props for [`MatRadio`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/radio#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/radio#events)
#[derive(Clone, Props, PartialEq)]
pub struct RadioProps {
    #[props(default)]
    pub checked: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(into)]
    pub name: Option<String>,
    #[props(into)]
    pub value: Option<String>,
    #[props(default)]
    pub global: bool,
    #[props(default)]
    pub reduced_touch_target: bool,
    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
}

#[component]
pub fn MatRadio(props: RadioProps) -> Element {
    let id = crate::use_id("radio");
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let radio = JsValue::from(elem).dyn_into::<Radio>().unwrap();
        radio.set_checked(props.checked);
    }
    rsx! {
        mwc-radio {
            id: id,

            disabled: props.disabled,
            name: props.name,
            value: props.value,
            global: props.global,
            reducedTouchTarget: props.reduced_touch_target,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
        }
    }
}

//component!('a, MatRadio, RadioProps, render, Radio, "radio");
