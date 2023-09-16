use crate::{list::request_selected::make_request_selected_listener, StaticCallback};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use super::{GraphicType, RequestSelectedDetail};

#[wasm_bindgen(module = "/build/mwc-radio-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type RadioListItem;

    #[wasm_bindgen(getter, static_method_of = RadioListItem)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_selected(this: &RadioListItem, value: bool);
}

loader_hack!(RadioListItem);

/// Props for [`MatRadioListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-radio-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-radio-list-item-2)
#[derive(Props)]
pub struct RadioListItemProps<'a> {
    #[props(default)]
    pub left: bool,
    #[props(into)]
    pub group: Option<String>,
    #[props(default = GraphicType::Control)]
    pub graphic: GraphicType,
    /// Binds to `request-selected` event on `mwc-list-item`.
    #[props(into)]
    pub _on_request_selected: Option<StaticCallback<RequestSelectedDetail>>,
    #[props(default)]
    pub initially_selected: bool,
    pub children: Element<'a>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
}

fn render<'a>(cx: Scope<'a, RadioListItemProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "radio-list-item");
    let request_selected_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem;
        if let Some(listener) = cx.props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }
    }

    render! {
        mwc-radio-list-item {
            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(id) {
                    let item = JsValue::from(elem).dyn_into::<RadioListItem>().unwrap();
                    item.set_selected(cx.props.initially_selected);
                }
            },

            id: id,

            left: bool_attr!(cx.props.left),
            graphic: cx.props.graphic.as_str(),
            group: optional_string_attr!(cx.props.group),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),

            &cx.props.children
        }
    }
}

component!('a, MatRadioListItem, RadioListItemProps, render, RadioListItem, "radio-list-item");
