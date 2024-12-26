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
#[derive(Clone, Props, PartialEq)]
pub struct RadioListItemProps {
    #[props(default)]
    pub left: bool,
    #[props(into)]
    pub group: Option<String>,
    #[props(default = GraphicType::Control)]
    pub graphic: GraphicType,
    /// Binds to `request-selected` event on `mwc-list-item`.
    #[props(default)]
    pub initially_selected: bool,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MatRadioListItem(props: RadioListItemProps) -> Element {
    let id = crate::use_id("radio-list-item");
    //let request_selected_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem;
        /*if let Some(listener) = props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }*/
    }

    rsx! {
        mwc-radio-list-item {
            id: id.clone(),

            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(&id) {
                    let item = JsValue::from(elem).dyn_into::<RadioListItem>().unwrap();
                    item.set_selected(props.initially_selected);
                }
            },


            left: props.left,
            graphic: props.graphic.as_str(),
            group: props.group,

            style: props.style,
            class: props.class,
        }
    }
}

//component!('a, MatRadioListItem, RadioListItemProps, render, RadioListItem, "radio-list-item");
