use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-icon.js")]
extern "C" {
    #[derive(Debug)]
    type Icon;

    #[wasm_bindgen(getter, static_method_of = Icon)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Icon);

/// Props for [`MatIcon`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon#propertiesattributes)
#[derive(Clone, Props, PartialEq)]
pub struct IconProps {
    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    pub children: Element,
}

#[component]
pub fn MatIcon(props: IconProps) -> Element {
    rsx! {
        mwc-icon {
            style: props.style,
            class: props.class,
            slot: props.slot,
            {props.children}
        }
    }
}

//component!('a, MatIcon, IconProps, render, Icon, "icon");
