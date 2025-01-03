#[cfg(feature = "textfield")]
mod textfield;
#[cfg(feature = "textfield")]
pub use textfield::*;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) mod validity_state;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use validity_state::ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) mod text_field_type;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use text_field_type::*;

#[cfg(feature = "textarea")]
mod textarea;
#[cfg(feature = "textarea")]
pub use textarea::*;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use web_sys::ValidityState as NativeValidityState;

use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::StaticCallback;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) type ValidityTransformFn = dyn Fn(String, NativeValidityState) -> ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
#[derive(Clone)]
/// Owned function for validity props
pub struct ValidityTransform(pub(crate) Rc<ValidityTransformFn>);

#[cfg(any(feature = "textfield", feature = "textarea"))]
impl ValidityTransform {
    pub fn new<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}

impl PartialEq for ValidityTransform {
    #[allow(clippy::vtable_address_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

fn set_on_input_handler(
    target: &web_sys::Element,
    callback: StaticCallback<String>,
    convert: impl Fn((Event, JsValue)) -> String + 'static,
) -> EventListener {
    EventListener::new(target, "input", move |event: &Event| {
        let js_value = JsValue::from(event);

        callback.call(convert((event.clone(), js_value)))
    })
}
