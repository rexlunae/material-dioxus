use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-formfield.js")]
extern "C" {
    #[derive(Debug)]
    type Formfield;

    #[wasm_bindgen(getter, static_method_of = Formfield)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Formfield);

/// Props for [`MatFormfield`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/formfield#propertiesattributes)
#[derive(Props)]
pub struct FormfieldProps<'a> {
    pub children: Element<'a>,
    #[props(into)]
    pub label: Option<String>,
    #[props(default)]
    pub align_end: bool,
    #[props(default)]
    pub space_between: bool,
    #[props(default)]
    pub nowrap: bool,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
}

fn render<'a>(cx: Scope<'a, FormfieldProps<'a>>) -> Element<'a> {
    render! {
        mwc-formfield {
            label: optional_string_attr!(cx.props.label),
            alignEnd: bool_attr!(cx.props.align_end),
            spaceBetween: bool_attr!(cx.props.space_between),
            nowrap: bool_attr!(cx.props.nowrap),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),
            dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),

            &cx.props.children
        }
    }
}

component!('a, MatFormfield, FormfieldProps, render, Formfield, "formfield");
