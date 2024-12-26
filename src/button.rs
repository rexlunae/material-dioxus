use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;

use crate::StaticCallback;

#[wasm_bindgen(module = "/build/mwc-button.js")]
extern "C" {
    #[derive(Debug)]
    type Button;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Button)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Button);

/// Props for [`MatButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/button#propertiesattributes)
#[derive(Clone, Props, PartialEq)]
pub struct ButtonProps {
    #[props(into)]
    pub label: String,
    #[props(into)]
    pub icon: Option<String>,
    // TODO: variant enum
    #[props(default)]
    pub raised: bool,
    #[props(default)]
    pub unelevated: bool,
    #[props(default)]
    pub outlined: bool,
    #[props(default)]
    pub dense: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub trailing_icon: bool,

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
pub fn MatButton(props: ButtonProps) -> Element {
    let id = crate::use_id("button");
    //let click_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem;
        /*if let Some(listener) = props._onclick.clone() {
             *click_listener = Some(EventListener::new(&target, "click", move |_| {
                listener.call(())
            }));
        }*/
    }

    rsx! {
        mwc-button {
            id: id,

            icon: props.icon,
            label: props.label,
            disabled: props.disabled,
            raised: props.raised,
            unelevated: props.unelevated,
            outlined: props.outlined,
            dense: props.dense,
            trailingIcon: props.trailing_icon,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
        }
    }
}

//component!('a, MatButton, ButtonProps, render, Button, "button");
