use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-icon-button.js")]
extern "C" {
    #[derive(Debug)]
    type IconButton;

    #[wasm_bindgen(getter, static_method_of = IconButton)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButton);

/// Props for [`MatIconButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button#propertiesattributes)
#[derive(Clone, Props, PartialEq)]
pub struct IconButtonProps {
    #[props(into)]
    pub label: Option<String>,
    #[props(into)]
    pub icon: Option<String>,
    #[props(default)]
    pub disabled: bool,
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
pub fn MatIconButton(props: IconButtonProps) -> Element {
    rsx! {
        mwc-icon-button {
            label: props.label,
            icon: props.icon,
            disabled: props.disabled,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
            {props.children}
        }

    }
}

//component!('a, MatIconButton, IconButtonProps, render, IconButton, "icon-button");
