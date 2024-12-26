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
#[derive(Clone, Props, PartialEq)]
pub struct SwitchProps {
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
    _lifetime: Option<PhantomData<()>>,

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
pub fn MatSwitch(props: SwitchProps) -> Element {
    let id = crate::use_id("switch");
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let _target = elem;
    }

    rsx! {
        mwc-switch {
            id: id,

            selected: props.selected,
            disabled: props.disabled,
            name: props.name,
            value: props.value,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
        }
    }
}
