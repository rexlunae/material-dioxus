use crate::{list::request_selected::make_request_selected_listener, StaticCallback};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use super::{GraphicType, RequestSelectedDetail};

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
#[derive(Props)]
pub struct ListItemProps<'a> {
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
    #[props(into)]
    pub _on_request_selected: Option<StaticCallback<RequestSelectedDetail>>,
    pub children: Element<'a>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
}

fn render<'a>(cx: Scope<'a, ListItemProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "list-item");
    let request_selected_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem;
        if let Some(listener) = cx.props._on_request_selected.clone() {
            *request_selected_listener = Some(make_request_selected_listener(&target, listener));
        }
    }

    render! {
        mwc-list-item {
            onmounted: move |_| {
                if let Some(elem) = crate::get_elem_by_id(id) {
                    let item = JsValue::from(elem).dyn_into::<ListItem>().unwrap();
                    item.set_activated(cx.props.initially_activated);
                    item.set_selected(cx.props.initially_selected);
                }
            },

            id: id,

            value: optional_string_attr!(cx.props.value),
            group: bool_attr!(cx.props.group),
            tabindex: cx.props.tabindex as i64,
            disabled: bool_attr!(cx.props.disabled),
            twoline: bool_attr!(cx.props.twoline),
            graphic: cx.props.graphic.as_str(),
            multipleGraphics: bool_attr!(cx.props.multiple_graphics),
            hasMeta: bool_attr!(cx.props.has_meta),
            noninteractive: bool_attr!(cx.props.noninteractive),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),

            &cx.props.children
        }
    }
}

component!('a, MatListItem, ListItemProps, render, ListItem, "list-item");
