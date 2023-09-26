use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;

use crate::StaticCallback;

#[wasm_bindgen(module = "/build/mwc-switch.js")]
extern "C" {
    #[derive(Debug)]
    type Switch;

    #[wasm_bindgen(getter, static_method_of = Switch)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Switch);

/// Props for [`MatSwitch`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/switch#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/switch#events)
#[derive(Props)]
pub struct SwitchProps<'a> {
    #[props(default)]
    pub selected: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(into)]
    pub name: Option<String>,
    #[props(into)]
    pub value: Option<String>,
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

fn render<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "switch");
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
        mwc-switch {
            id: id,

            selected: bool_attr!(cx.props.selected),
            disabled: bool_attr!(cx.props.disabled),
            name: optional_string_attr!(cx.props.name),
            value: optional_string_attr!(cx.props.value),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),
            dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),
        }
    }
}

component!('a, MatSwitch, SwitchProps, render, Switch, "switch");
