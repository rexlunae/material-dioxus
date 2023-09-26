use dioxus::{core::AttributeValue, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-circular-progress-four-color.js")]
extern "C" {
    #[derive(Debug)]
    type CircularProgressFourColor;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CircularProgressFourColor)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CircularProgressFourColor);

/// Props for [`MatCircularProgressFourColor`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/circular-progress-four-color#propertiesattributes)
#[derive(Props, PartialEq)]
pub struct CircularProgressFourColorProps {
    #[props(default)]
    pub indeterminate: bool,
    #[props(default)]
    pub progress: f32,
    #[props(default)]
    pub density: u32,
    #[props(default)]
    pub closed: bool,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
}

fn render(cx: Scope<CircularProgressFourColorProps>) -> Element {
    render! {
        mwc-circular-progress-four-color {
            indeterminate: bool_attr!(cx.props.indeterminate),
            progress: AttributeValue::Float(cx.props.progress.into()),
            density: AttributeValue::Int(cx.props.density.into()),
            closed: bool_attr!(cx.props.closed),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),
        }
    }
}

component!(
    MatCircularProgressFourColor,
    CircularProgressFourColorProps,
    render,
    CircularProgressFourColor,
    "circular-progress-four-color"
);
