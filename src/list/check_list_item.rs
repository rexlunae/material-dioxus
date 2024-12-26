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
#[derive(Clone, Props, PartialEq)]
pub struct CheckListItemProps {
    #[props(default)]
    pub left: bool,
    #[props(default = GraphicType::Control)]
    pub graphic: GraphicType,
    #[props(default)]
    pub disabled: bool,
    //#[props(into)]
    //pub _on_request_selected: Option<StaticCallback<RequestSelectedDetail>>,
    #[props(default)]
    pub initially_selected: bool,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MatCheckListItem(props: CheckListItemProps) -> Element {
    let id = crate::use_id("check-list-item");
    //let request_selected_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem;
        /*if let Some(listener) = props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }*/
    }

    rsx! {
        mwc-check-list-item {
            id: id.clone(),

            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(&id) {
                    let item = JsValue::from(elem).dyn_into::<CheckListItem>().unwrap();
                    item.set_selected(props.initially_selected);
                }
            },

            left: props.left,
            graphic: props.graphic.as_str(),
            disabled: props.disabled,

            style: props.style,
            class: props.class,
            {props.children}
        }
    }
}

//component!('a, MatCheckListItem, CheckListItemProps, render, CheckListItem, "check-list-item");
