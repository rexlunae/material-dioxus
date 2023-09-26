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
#[derive(Props)]
pub struct RadioProps<'a> {
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
    /// Binds to `change`.
    ///
    /// Callback's parameter of type denotes if the radio is checked or not.
    ///
    /// See events docs to learn more.
    #[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    pub _onchange: Option<StaticCallback<bool>>,
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

fn render<'a>(cx: Scope<'a, RadioProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "radio");
    let change_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem.clone();
        let radio = JsValue::from(elem).dyn_into::<Radio>().unwrap();
        radio.set_checked(cx.props.checked);
        if let Some(listener) = cx.props._onchange.clone() {
            *change_listener = Some(EventListener::new(&target, "change", move |_| {
                listener.call(radio.checked())
            }));
        }
    }
    render! {
        mwc-radio {
            id: id,

            disabled: bool_attr!(cx.props.disabled),
            name: optional_string_attr!(cx.props.name),
            value: optional_string_attr!(cx.props.value),
            global: bool_attr!(cx.props.global),
            reducedTouchTarget: bool_attr!(cx.props.reduced_touch_target),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),
            dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),
        }
    }
}

component!('a, MatRadio, RadioProps, render, Radio, "radio");
