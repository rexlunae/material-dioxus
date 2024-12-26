use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use super::GraphicType;

#[wasm_bindgen(module = "/build/mwc-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type ListItem;

    #[wasm_bindgen(getter, static_method_of = ListItem)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_activated(this: &ListItem, value: bool);

    #[wasm_bindgen(method, setter)]
    fn set_selected(this: &ListItem, value: bool);
}

loader_hack!(ListItem);

/// Props for [`MatListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-2)
#[derive(Clone, Props, PartialEq)]
pub struct ListItemProps {
    #[props(into)]
    pub value: Option<String>,
    #[props(default)]
    pub group: bool,
    #[props(default = -1)]
    pub tabindex: i32,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub twoline: bool,
    #[props(default)]
    pub initially_activated: bool,
    #[props(default = GraphicType::Null)]
    pub graphic: GraphicType,
    #[props(default)]
    pub multiple_graphics: bool,
    #[props(default)]
    pub has_meta: bool,
    #[props(default)]
    pub noninteractive: bool,
    #[props(default)]
    pub initially_selected: bool,
    /// Binds to `request-selected` event on `mwc-list-item`.
    //#[props(into)]
    //pub _on_request_selected: Option<StaticCallback<RequestSelectedDetail>>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MatListItem(props: ListItemProps) -> Element {
    let id = crate::use_id("list-item");
    //let request_selected_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem;
        /*if let Some(listener) = props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }*/
    }

    rsx! {
        mwc-list-item {
            id: id.clone(),

            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(&id) {
                    let item = JsValue::from(elem).dyn_into::<ListItem>().unwrap();
                    item.set_activated(props.initially_activated);
                    item.set_selected(props.initially_selected);
                }
            },

            value: props.value,
            group: props.group,
            tabindex: props.tabindex as i64,
            disabled: props.disabled,
            twoline: props.twoline,
            graphic: props.graphic.as_str(),
            multipleGraphics: props.multiple_graphics,
            hasMeta: props.has_meta,
            noninteractive: props.noninteractive,

            style: props.style,
            class: props.class,
            {props.children}
        }
    }
}
