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
#[derive(Props)]
pub struct FabProps<'a> {
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
    #[props(default)]
    pub children: Element<'a>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
}

fn render<'a>(cx: Scope<'a, FabProps<'a>>) -> Element<'a> {
    match &cx.props.children {
        Some(children) => {
            render! {
                mwc-fab {
                    label: optional_string_attr!(cx.props.label),
                    icon: optional_string_attr!(cx.props.icon),
                    mini: bool_attr!(cx.props.mini),
                    reducedTouchTarget: bool_attr!(cx.props.reduced_touch_target),
                    extended: bool_attr!(cx.props.extended),
                    showIconAtEnd: bool_attr!(cx.props.show_icon_at_end),

                    style: string_attr!(cx.props.style),
                    class: string_attr!(cx.props.class),
                    slot: optional_string_attr!(cx.props.slot),
                    dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),

                    children
                }
            }
        }
        None => {
            render! {
                mwc-fab {
                    label: optional_string_attr!(cx.props.label),
                    icon: optional_string_attr!(cx.props.icon),
                    mini: bool_attr!(cx.props.mini),
                    reducedTouchTarget: bool_attr!(cx.props.reduced_touch_target),
                    extended: bool_attr!(cx.props.extended),
                    showIconAtEnd: bool_attr!(cx.props.show_icon_at_end),

                    style: string_attr!(cx.props.style),
                    class: string_attr!(cx.props.class),
                    slot: optional_string_attr!(cx.props.slot),
                    dialogInitialFocus: bool_attr!(cx.props.dialog_initial_focus),
                }
            }
        }
    }
}

component!('a, MatFab, FabProps, render, Fab, "fab");
