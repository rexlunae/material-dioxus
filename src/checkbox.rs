use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::utils::StaticCallback;

#[wasm_bindgen(module = "/build/mwc-checkbox.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Checkbox;

    #[wasm_bindgen(getter, static_method_of = Checkbox)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Checkbox, value: bool);

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Checkbox) -> bool;
}

loader_hack!(Checkbox);

/// Props for [`MatCheckbox`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/checkbox#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/checkbox#events)
#[derive(Clone, Props, PartialEq)]
pub struct CheckboxProps {
    #[props(default)]
    pub checked: bool,
    #[props(default)]
    pub indeterminate: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(into)]
    pub value: Option<String>,
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
pub fn MatCheckbox(props: CheckboxProps) -> Element {
    let id = crate::use_id("checkbox");
    //let change_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem.clone();
        let cb = JsValue::from(elem).dyn_into::<Checkbox>().unwrap();
        cb.set_checked(props.checked);
        /*
        if let Some(listener) = props._onchange.clone() {
            *change_listener = Some(EventListener::new(&target, "change", move |_| {
                listener.call(cb.checked())
            }));
        }*/
    }
    rsx! {
        mwc-checkbox {
            id: id,

            indeterminate: props.indeterminate,
            disabled: props.disabled,
            value: props.value,
            reducedTouchTarget: props.reduced_touch_target,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
        }
    }
}

//component!('a, MatCheckbox, CheckboxProps, render, Checkbox, "checkbox");
