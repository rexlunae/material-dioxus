use crate::{event_details_into, StaticCallback};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;

/// Type for [`RequestSelectedDetail::source`]
#[derive(Debug, Clone)]
pub enum RequestSelectedSource {
    Interaction,
    Property,
}

/// The `RequestSelectedDetail` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-2)
#[derive(Debug, Clone)]
pub struct RequestSelectedDetail {
    pub selected: bool,
    pub source: RequestSelectedSource,
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type RequestSelectedDetailJS;

    #[wasm_bindgen(method, getter)]
    fn selected(this: &RequestSelectedDetailJS) -> bool;

    #[wasm_bindgen(method, getter)]
    fn source(this: &RequestSelectedDetailJS) -> String;
}

pub fn make_request_selected_listener(
    target: &web_sys::Element,
    callback: StaticCallback<RequestSelectedDetail>,
) -> EventListener {
    EventListener::new(target, "request-selected", move |event| {
        let selected_detail = event_details_into::<RequestSelectedDetailJS>(event);
        let selected_detail = RequestSelectedDetail {
            selected: selected_detail.selected(),
            source: match selected_detail.source().as_str() {
                "interaction" => RequestSelectedSource::Interaction,
                "property" => RequestSelectedSource::Property,
                val => {
                    panic!(
                        "invalid `source` value {} received. This should never happen",
                        val
                    )
                }
            },
        };
        callback.call(selected_detail);
    })
}
