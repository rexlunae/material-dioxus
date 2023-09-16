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
#[derive(Props)]
pub struct IconProps<'a> {
    pub children: Element<'a>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
}

fn render<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    render! {
        mwc-icon {
            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),

            &cx.props.children
        }
    }
}

component!('a, MatIcon, IconProps, render, Icon, "icon");
