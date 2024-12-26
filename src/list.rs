mod list_item;
pub use list_item::*;

mod check_list_item;
pub use check_list_item::*;

mod radio_list_item;
pub use radio_list_item::*;

mod separator;
pub use separator::*;

mod list_index;
pub use list_index::ListIndex;

mod selected_detail;
pub use selected_detail::{IndexDiff, SelectedDetail};

mod action_detail;
pub use action_detail::ActionDetail;

mod request_selected;
pub use request_selected::{RequestSelectedDetail, RequestSelectedSource};

mod graphic_type;
pub use graphic_type::GraphicType;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::{event_into_details, StaticCallback};

#[wasm_bindgen(module = "/build/mwc-list.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type List;

    #[wasm_bindgen(getter, static_method_of = List)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn index(this: &List) -> JsValue;

    #[wasm_bindgen(method)]
    fn toggle(this: &List, index: usize, force: bool);

    #[wasm_bindgen(method, js_name = getFocusedItemIndex)]
    fn get_focused_item_index(this: &List) -> usize;

    #[wasm_bindgen(method, js_name = focusItemAtIndex)]
    fn focus_item_at_index(this: &List, index: usize);
}

loader_hack!(List);

/// Props for [`MatList`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-1)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-2)
#[derive(Clone, Props, PartialEq)]
pub struct ListProps {
    #[props(default)]
    pub activatable: bool,
    #[props(default)]
    pub root_tabbable: bool,
    #[props(default)]
    pub multi: bool,
    #[props(default)]
    pub wrap_focus: bool,
    #[props(into)]
    pub item_roles: Option<String>,
    #[props(into)]
    pub inner_role: Option<String>,
    #[props(default)]
    pub noninteractive: bool,
    // TODO: make methods callable
    // /// [`WeakComponentLink`] for `MatList` which provides the following methods
    // /// - ```toggle(&self, index: usize, force: bool)```
    // /// - ```get_focused_item_index(&self) -> usize```
    // /// - ```focus_item_at_index(&self, index: usize)```
    // ///
    // /// See [`WeakComponentLink`] documentation for more information
    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    pub children: Element,
}

#[component]
pub fn MatList(props: ListProps) -> Element {
    let id = crate::use_id("list");
    //let selected_listener = use_hook(|| None);
    //let action_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let _target = elem.clone();
        let _list = JsValue::from(elem).dyn_into::<List>().unwrap();
    }

    rsx! {
        mwc-list {
            id: id,

            activatable: props.activatable,
            rootTabbable: props.root_tabbable,
            multi: props.multi,
            wrapFocus: props.wrap_focus,
            itemRoles: props.item_roles,
            innerRole: props.inner_role,
            noninteractive: props.noninteractive,

            style: props.style,
            class: props.class,
            slot: props.slot,
            {props.children}
        }
    }
}
