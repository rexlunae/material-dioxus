use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-fab.js")]
extern "C" {
    #[derive(Debug)]
    type Fab;

    #[wasm_bindgen(getter, static_method_of = Fab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Fab);

/// Props for [`MatFab`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/fab#propertiesattributes)
#[derive(Clone, Props, PartialEq)]
pub struct FabProps {
    #[props(into)]
    pub icon: Option<String>,
    #[props(into)]
    pub label: Option<String>,
    #[props(default)]
    pub mini: bool,
    #[props(default)]
    pub reduced_touch_target: bool,
    #[props(default)]
    pub extended: bool,
    #[props(default)]
    pub show_icon_at_end: bool,

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
pub fn MatFab(props: FabProps) -> Element {
    rsx! {
        mwc-fab {
            label: props.label,
            icon: props.icon,
            mini: props.mini,
            reducedTouchTarget: props.reduced_touch_target,
            extended: props.extended,
            showIconAtEnd: props.show_icon_at_end,

            style: props.style,
            class: props.class,
            slot: props.slot,
            dialogInitialFocus: props.dialog_initial_focus,
            {props.children}
        }
    }
}

//component!('a, MatFab, FabProps, render, Fab, "fab");
