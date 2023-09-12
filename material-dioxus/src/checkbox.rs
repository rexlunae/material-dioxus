use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo::events::EventListener;
use rand::distributions::{Alphanumeric, DistString};
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
#[derive(Props)]
pub struct CheckboxProps<'a> {
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
    /// Binds to `change` event on `mwc-checkbox`
    ///
    /// See events docs to learn more.
    #[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    pub _onchange: Option<StaticCallback<bool>>,
    _lifetime: Option<PhantomData<&'a ()>>,
}

fn render<'a>(cx: Scope<'a, CheckboxProps<'a>>) -> Element<'a> {
    let id = cx
        .use_hook(|| {
            let mut id = String::from("checkbox-");
            Alphanumeric.append_string(&mut rand::thread_rng(), &mut id, 11);
            // make sure an initial value of `checked = true` is properly set
            cx.needs_update();
            id
        })
        .as_str();
    let change_listener = cx.use_hook(|| None);
    if let Some(elem) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
    {
        let target = elem.clone();
        let cb = JsValue::from(elem).dyn_into::<Checkbox>().unwrap();
        cb.set_checked(cx.props.checked);
        if let Some(listener) = cx.props._onchange.clone() {
            *change_listener = Some(EventListener::new(&target, "change", move |_| {
                listener.call(cb.checked())
            }));
        }
    }
    render! {
        mwc-checkbox {
            id: id,
            "indeterminate": bool_attr!(cx.props.indeterminate),
            "disabled": cx.props.disabled,
            "value": optional_string_attr!(cx.props.value),
            "reducedTouchTarget": bool_attr!(cx.props.reduced_touch_target),
        }
    }
}

component!('a, MatCheckbox, CheckboxProps, render, Checkbox, "checkbox");
