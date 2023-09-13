use std::marker::PhantomData;

use super::set_on_input_handler;
use crate::text_inputs::{validity_state::ValidityStateJS, TextFieldType, ValidityTransform};
use crate::StaticCallback;
use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use web_sys::ValidityState as NativeValidityState;

#[rustfmt::skip]
// mod dioxus_elements {
//     pub use dioxus::prelude::dioxus_elements::*;
//
//     // pub mod events {
//     //     use super::super::MatTextFieldInputEvent;
//     //     pub use dioxus::prelude::dioxus_elements::events::*;
//     //
//     //     #[inline]
//     //     pub(in super::super) fn oninput<'a, E: dioxus::html::EventReturn<T>, T>(
//     //         cx: &'a dioxus::core::ScopeState,
//     //         mut f: impl FnMut(dioxus::core::Event<MatTextFieldInputEvent>) -> E + 'a,
//     //     ) -> dioxus::core::Attribute<'a> {
//     //         dioxus::core::Attribute::new(
//     //             "oninput",
//     //             cx.listener(move |e: dioxus::core::Event<MatTextFieldInputEvent>| {
//     //                 f(e).spawn(cx);
//     //             }),
//     //             Some("mwc-textfield"),
//     //             false,
//     //         )
//     //     }
//     // }
//
//     pub struct mwctextfield;
//     impl mwctextfield {
//         pub const TAG_NAME: &str = "mwc-textfield";
//         pub const NAME_SPACE: Option<&str> = Some("mwc-textfield");
//
//         pub const open: (&str, Option<&str>, bool) = ("open", Some("mwc-textfield"), false);
//     }
//     impl GlobalAttributes for mwctextfield {}
// }

#[wasm_bindgen(module = "/build/mwc-textfield.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextField;

    #[wasm_bindgen(getter, static_method_of = TextField)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextField,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextField, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextField) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextField, val: &JsValue);
}

loader_hack!(TextField);

/// Props for [`MatTextField`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/textfield#propertiesattributes)
#[derive(Props)]
pub struct TextFieldProps<'a> {
    #[props(default)]
    pub open: bool,
    #[props(into)]
    // TODO: really Option?
    pub value: Option<String>,
    #[props(default = TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[props(into)]
    pub label: Option<String>,
    #[props(into)]
    pub placeholder: Option<String>,
    #[props(into)]
    pub prefix: Option<String>,
    #[props(into)]
    pub suffix: Option<String>,
    #[props(into)]
    pub icon: Option<String>,
    #[props(into)]
    pub icon_trailing: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub char_counter: bool,
    #[props(default)]
    pub outlined: bool,
    #[props(into)]
    pub helper: Option<String>,
    #[props(default)]
    pub helper_persistent: bool,
    #[props(default)]
    pub required: bool,
    #[props(default)]
    pub max_length: Option<u64>,
    #[props(into)]
    pub validation_message: Option<String>,
    #[props(into)]
    pub pattern: Option<String>,
    /// Type: `number | string` so I'll leave it as a string
    #[props(into)]
    pub min: Option<String>,
    /// Type: `number | string`  so I'll leave it as a string
    #[props(into)]
    pub max: Option<String>,
    // What you doing...
    #[props(default)]
    pub size: Option<i64>,
    // ...step size
    #[props(default)]
    pub step: Option<i64>,
    #[props(default)]
    pub auto_validate: bool,
    pub validity_transform: Option<ValidityTransform>,
    #[props(default)]
    pub validate_on_initial_render: bool,
    #[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    pub _oninput: Option<StaticCallback<String>>,
    _lifetime: Option<PhantomData<&'a ()>>,
    #[props(into)]
    pub name: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
}

fn render<'a>(cx: Scope<'a, TextFieldProps<'a>>) -> Element<'a> {
    let id = cx
        .use_hook(|| {
            let mut id = String::from("textfield-");
            Alphanumeric.append_string(&mut rand::thread_rng(), &mut id, 11);
            // make sure all initial values are properly set
            cx.needs_update();
            id
        })
        .as_str();
    let input_listener = cx.use_hook(|| None);
    let validity_transform_closure = cx.use_hook(|| None);
    if let Some(elem) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
    {
        let textfield = JsValue::from(elem).dyn_into::<TextField>().unwrap();
        textfield.set_type(&JsValue::from(cx.props.field_type.as_str()));
        textfield.set_value(&JsValue::from_str(
            cx.props
                .value
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or_default(),
        ));
        if let Some(listener) = cx.props._oninput.clone() {
            *input_listener = set_on_input_handler(id, listener, |(_, detail)| {
                detail
                    .unchecked_into::<MatTextFieldInputEvent>()
                    .target()
                    .value()
            });
        }
        if let (Some(transform), None) = (
            cx.props.validity_transform.clone(),
            &validity_transform_closure,
        ) {
            *validity_transform_closure = Some(Closure::wrap(Box::new(
                move |s: String, v: NativeValidityState| -> ValidityStateJS {
                    transform.0(s, v).into()
                },
            )
                as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
            textfield.set_validity_transform(validity_transform_closure.as_ref().unwrap());
        }
    }
    render! {
        mwc-textfield {
            id: id,
            // open: bool_attr!(cx.props.open),
            "open": bool_attr!(cx.props.open),
            "label": optional_string_attr!(cx.props.label),
            "placeholder": optional_string_attr!(cx.props.placeholder),
            "prefix": optional_string_attr!(cx.props.prefix),
            "suffix": optional_string_attr!(cx.props.suffix),
            "icon": optional_string_attr!(cx.props.icon),
            "iconTrailing": optional_string_attr!(cx.props.icon_trailing),
            "disabled": bool_attr!(cx.props.disabled),
            "charCounter": bool_attr!(cx.props.char_counter),
            "outlined": bool_attr!(cx.props.outlined),
            "helper": optional_string_attr!(cx.props.helper),
            "helperPersistent": bool_attr!(cx.props.helper_persistent),
            "required": bool_attr!(cx.props.required),
            "maxLength": cx.props.max_length.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            "validationMessage": optional_string_attr!(cx.props.validation_message),
            "pattern": optional_string_attr!(cx.props.pattern),
            "min": optional_string_attr!(cx.props.min),
            "max": optional_string_attr!(cx.props.max),
            "size": cx.props.size.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            "step": cx.props.step.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            "autoValidate": bool_attr!(cx.props.auto_validate),
            "validateOnInitialRender": bool_attr!(cx.props.validate_on_initial_render),
            "name": optional_string_attr!(cx.props.name),
            "dialogInitialFocus": bool_attr!(cx.props.dialog_initial_focus),
        }
    }
}

component!('a, MatTextField, TextFieldProps, render, TextField, "textfield");

#[wasm_bindgen]
extern "C" {
    type MatTextFieldInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextFieldInputEvent) -> TextField;
}
