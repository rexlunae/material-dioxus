mod dialog_action;

pub use dialog_action::*;

use dioxus::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use crate::StaticCallback;

#[wasm_bindgen(module = "/build/mwc-dialog.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Dialog;

    #[wasm_bindgen(getter, static_method_of = Dialog)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn focus(this: &Dialog);

    #[wasm_bindgen(method)]
    fn blur(this: &Dialog);

    #[wasm_bindgen(method)]
    fn show(this: &Dialog);

    #[wasm_bindgen(method)]
    fn close(this: &Dialog);
}

loader_hack!(Dialog);

// /// The `mwc-dialog` component.
// ///
// /// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog)
// ///
// /// ## Actions
// ///
// /// In order to pass actions, [`MatDialogAction`] component should be
// /// used.
// pub struct MatDialog {
//     node_ref: NodeRef,
//     opening_listener: Option<EventListener>,
//     opened_listener: Option<EventListener>,
//     closing_listener: Option<EventListener>,
//     closed_listener: Option<EventListener>,
// }

/// Props for [`MatDialog`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog#events)
#[derive(Clone, Props, PartialEq)]
pub struct DialogProps {
    #[props(default)]
    pub open: bool,
    #[props(default)]
    pub hide_actions: bool,
    #[props(default)]
    pub stacked: bool,
    #[props(into)]
    pub heading: Option<String>,
    #[props(into)]
    pub scrim_click_action: Option<String>,
    #[props(into)]
    pub escape_key_action: Option<String>,
    #[props(into)]
    pub default_action: Option<String>,
    #[props(into)]
    pub action_attribute: Option<String>,
    #[props(into)]
    pub initial_focus_attribute: Option<String>,
    /// Binds to `opening` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    //#[props(into)]
    //pub _onopening: Option<StaticCallback<()>>,
    /// Binds to `opened` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    //#[props(into)]
    //pub _onopened: Option<StaticCallback<()>>,
    /// Binds to `closing` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    //#[props(into)]
    //pub _onclosing: Option<StaticCallback<String>>,
    /// Binds to `closed` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    //#[props(into)]
    //pub _onclosed: Option<StaticCallback<String>>,
    // TODO: make methods callable
    // /// [`WeakComponentLink`] for `MatDialog` which provides the following
    // /// methods:
    // /// - ```focus(&self)```
    // /// - ```blur(&self)```
    // /// - ```show(&self)```
    // /// - ```close(&self)```
    // ///
    // /// See [`WeakComponentLink`] documentation for more information

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn MatDialog(props: DialogProps) -> Element {
    let id = crate::use_id("dialog");
    //let opening_listener = use_hook(|| None);
    //let opened_listener = use_hook(|| None);
    //let closing_listener = use_hook(|| None);
    //let closed_listener = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem;
        /*
        if let Some(listener) = props._onopening.clone() {
            *opening_listener = Some(EventListener::new(&target, "opening", move |_| {
                listener.call(())
            }));
        }
        if let Some(listener) = props._onopened.clone() {
            *opened_listener = Some(EventListener::new(&target, "opened", move |_| {
                listener.call(())
            }));
        }
        if let Some(listener) = props._onclosing.clone() {
            *closing_listener = Some(EventListener::new(&target, "closing", move |event| {
                listener.call(action_from_event(event))
            }));
        }
        if let Some(listener) = props._onclosed.clone() {
            *closed_listener = Some(EventListener::new(&target, "closed", move |event| {
                listener.call(action_from_event(event))
            }));
        }*/
    }
    rsx! {
        mwc-dialog {
            id: id,

            open: props.open,
            hideActions: props.hide_actions,
            stacked: props.stacked,
            heading: props.heading,
            scrimClickAction: props.scrim_click_action,
            escapeKeyAction: props.escape_key_action,
            defaultAction: props.default_action,
            actionAttribute: props.action_attribute,
            initialFocusAttribute: props.initial_focus_attribute,

            style: "position: absolute; {props.style}",
            class: props.class,
            {props.children}
        }
    }
}

#[wasm_bindgen]
extern "C" {
    type DialogActionType;

    #[wasm_bindgen(method, getter)]
    fn action(this: &DialogActionType) -> String;
}

fn action_from_event(event: &web_sys::Event) -> String {
    crate::event_details_into::<DialogActionType>(event).action()
}
