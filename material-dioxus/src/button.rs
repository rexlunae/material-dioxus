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
#[derive(Props)]
pub struct ButtonProps<'a> {
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

    #[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    pub _onclick: Option<StaticCallback<()>>,
    _lifetime: Option<PhantomData<&'a ()>>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
}

fn render<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "button");
    let click_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem;
        if let Some(listener) = cx.props._onclick.clone() {
            *click_listener = Some(EventListener::new(&target, "click", move |_| {
                listener.call(())
            }));
        }
    }

    render! {
        mwc-button {
            id: id,

            icon: optional_string_attr!(cx.props.icon),
            label: string_attr!(cx.props.label),
            disabled: bool_attr!(cx.props.disabled),
            raised: bool_attr!(cx.props.raised),
            unelevated: bool_attr!(cx.props.unelevated),
            outlined: bool_attr!(cx.props.outlined),
            dense: bool_attr!(cx.props.dense),
            trailingIcon: bool_attr!(cx.props.trailing_icon),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),
            dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),
        }
    }
}

component!('a, MatButton, ButtonProps, render, Button, "button");
