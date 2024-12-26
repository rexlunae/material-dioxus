use dioxus::prelude::*;
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
#[derive(Clone, Props, PartialEq)]
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

#[component]
pub fn MatCircularProgressFourColor(props: CircularProgressFourColorProps) -> Element {
    rsx! {
        mwc-circular-progress-four-color {
            indeterminate: props.indeterminate,
            progress: props.progress,
            density: props.density,
            closed: props.closed,

            style: props.style,
            class: props.class,
            slot: props.slot,
        }
    }
}

/*component!(
    MatCircularProgressFourColor,
    CircularProgressFourColorProps,
    render,
    CircularProgressFourColor,
    "circular-progress-four-color"
);*/
