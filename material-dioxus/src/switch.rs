use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

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
#[derive(Props, PartialEq)]
pub struct SwitchProps {
    #[props(default)]
    pub selected: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(into)]
    pub name: Option<String>,
    #[props(into)]
    pub value: Option<String>,
}

fn render(cx: Scope<SwitchProps>) -> Element {
    render! {
        mwc-switch {
            "selected": bool_attr!(cx.props.selected),
            "disabled": bool_attr!(cx.props.disabled),
            "name": optional_string_attr!(cx.props.name),
            "value": optional_string_attr!(cx.props.value),
        }
    }
}

component!(MatSwitch, SwitchProps, render, Switch, "switch");
