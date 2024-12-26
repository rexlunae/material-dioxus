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
#[derive(Clone, Props, PartialEq)]
pub struct FormfieldProps {
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
    pub children: Element,
}

#[component]
pub fn MatFormfield(props: FormfieldProps) -> Element {
    rsx! {
        mwc-formfield {
            label: props.label,
            alignEnd: props.align_end,
            spaceBetween: props.space_between,
            nowrap: props.nowrap,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
            {props.children}
        }
    }
}

//component!('a, MatFormfield, FormfieldProps, render, Formfield, "formfield");
