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
#[derive(Props)]
pub struct ListProps<'a> {
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
    /// Binds to `action` event on `mwc-list`
    #[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    pub _onaction: Option<StaticCallback<ListIndex>>,
    /// Binds to `selected` event `mwc-list`
    #[props(into)]
    pub _onselected: Option<StaticCallback<SelectedDetail>>,
    // TODO: make methods callable
    // /// [`WeakComponentLink`] for `MatList` which provides the following methods
    // /// - ```toggle(&self, index: usize, force: bool)```
    // /// - ```get_focused_item_index(&self) -> usize```
    // /// - ```focus_item_at_index(&self, index: usize)```
    // ///
    // /// See [`WeakComponentLink`] documentation for more information
    // #[props(default)]
    // pub list_link: WeakComponentLink<MatList>,
    pub children: Element<'a>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
}

fn render<'a>(cx: Scope<'a, ListProps<'a>>) -> Element<'a> {
    let id = crate::use_id(cx, "list");
    let selected_listener = cx.use_hook(|| None);
    let action_listener = cx.use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(id) {
        let target = elem.clone();
        let list = JsValue::from(elem).dyn_into::<List>().unwrap();
        if let Some(listener) = cx.props._onselected.clone() {
            *selected_listener = Some(EventListener::new(&target, "selected", move |event| {
                let val = SelectedDetail::from(event_into_details(event));
                listener.call(val)
            }));
        }
        if let Some(listener) = cx.props._onaction.clone() {
            *action_listener = Some(EventListener::new(&target, "action", move |_| {
                let val: JsValue = list.index();
                let index = ListIndex::from(val);
                listener.call(index)
            }));
        }
    }

    render! {
        mwc-list {
            id: id,

            activatable: bool_attr!(cx.props.activatable),
            rootTabbable: bool_attr!(cx.props.root_tabbable),
            multi: bool_attr!(cx.props.multi),
            wrapFocus: bool_attr!(cx.props.wrap_focus),
            itemRoles: optional_string_attr!(cx.props.item_roles),
            innerRole: optional_string_attr!(cx.props.inner_role),
            noninteractive: bool_attr!(cx.props.noninteractive),

            style: string_attr!(cx.props.style),
            class: string_attr!(cx.props.class),
            slot: optional_string_attr!(cx.props.slot),

            &cx.props.children
        }
    }
}

component!('a, MatList, ListProps, render, List, "list");
