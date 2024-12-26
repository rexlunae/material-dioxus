use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-circular-progress.js")]
extern "C" {
    #[derive(Debug)]
    type CircularProgress;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CircularProgress)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CircularProgress);

/// Props for [`MatCircularProgress`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/circular-progress#propertiesattributes)
#[derive(Clone, Props, PartialEq)]
pub struct CircularProgressProps {
    #[props(default)]
    pub indeterminate: bool,
    #[props(default)]
    pub progress: f32,
    #[props(default)]
    pub density: i64,
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
pub fn MatCircularProgress(props: CircularProgressProps) -> Element {
    rsx! {
        mwc-circular-progress {
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
    MatCircularProgress,
    CircularProgressProps,
    render,
    CircularProgress,
    "circular-progress"
);*/
