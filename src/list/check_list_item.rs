use crate::{list::request_selected::make_request_selected_listener, StaticCallback};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use super::{GraphicType, RequestSelectedDetail};

#[wasm_bindgen(module = "/build/mwc-check-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type CheckListItem;

    #[wasm_bindgen(getter, static_method_of = CheckListItem)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_selected(this: &CheckListItem, value: bool);
}

loader_hack!(CheckListItem);

/// Props for [`MatCheckListItem`]
///
/// MWC Documentation for [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-check-list-item)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-check-list-item-1)
#[derive(Props)]
pub struct CheckListItemProps<'a> {
    #[props(default)]
    pub left: bool,
    #[props(default = GraphicType::Control)]
    pub graphic: GraphicType,
    #[props(default)]
    pub disabled: bool,
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

fn render<'a>(cx: Scope<'a, CheckListItemProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "check-list-item");
    let request_selected_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem;
        if let Some(listener) = cx.props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }
    }

    render! {
        mwc-check-list-item {
            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(id) {
                    let item = JsValue::from(elem).dyn_into::<CheckListItem>().unwrap();
                    item.set_selected(cx.props.initially_selected);
                }
            },

            id: id,

            left: bool_attr!(cx.props.left),
            graphic: cx.props.graphic.as_str(),
            disabled: bool_attr!(cx.props.disabled),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),

            &cx.props.children
        }
    }
}

component!('a, MatCheckListItem, CheckListItemProps, render, CheckListItem, "check-list-item");
