use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

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
#[derive(Props, PartialEq)]
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
}

fn render(cx: Scope<ButtonProps>) -> Element {
    render! {
        mwc-button {
            "icon": optional_string_attr!(cx.props.icon),
            "label": string_attr!(cx.props.label),
            "disabled": bool_attr!(cx.props.disabled),
            "raised": bool_attr!(cx.props.raised),
            "unelevated": bool_attr!(cx.props.unelevated),
            "outlined": bool_attr!(cx.props.outlined),
            "dense": bool_attr!(cx.props.dense),
            "trailingIcon": bool_attr!(cx.props.trailing_icon),
        }
    }
}

component!(MatButton, ButtonProps, render, Button, "button");
