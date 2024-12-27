use std::fmt;

use crate::text_inputs::validity_state::ValidityStateJS;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use web_sys::ValidityState as NativeValidityState;

use super::TextFieldType;
use super::ValidityTransform;

#[wasm_bindgen(module = "/build/mwc-textarea.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextArea;

    #[wasm_bindgen(getter, static_method_of = TextArea)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextArea,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextArea, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextArea) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextArea, val: &JsValue);
}

loader_hack!(TextArea);

/// Type for [`TextAreaProps::char_counter`].
///
/// Equivalent to `type TextAreaCharCounter = 'external'|'internal';` Typescript
/// type.
#[derive(Clone, Copy, PartialEq)]
pub enum TextAreaCharCounter {
    Internal,
    External,
}

impl TextAreaCharCounter {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextAreaCharCounter::Internal => "internal",
            TextAreaCharCounter::External => "external",
        }
    }
}

impl fmt::Display for TextAreaCharCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Props for [`MatTextArea`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/checkbox#propertiesattributes)
// TODO: why do the props icon, iconTrailing, field_type and so on exist?
#[derive(Clone, Props, PartialEq)]
pub struct TextAreaProps {
    #[props(default)]
    pub rows: Option<i64>,
    #[props(default)]
    pub cols: Option<i64>,
    #[props(into)]
    pub value: Option<String>,
    #[props(default = TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[props(into)]
    pub label: Option<String>,
    #[props(into)]
    pub placeholder: Option<String>,
    #[props(into)]
    pub icon: Option<String>,
    #[props(into)]
    pub icon_trailing: Option<String>,
    #[props(default)]
    pub disabled: bool,
    /// For boolean value `true`, `TextAreaCharCounter::External` is to be used.
    /// Boolean value `false` results in character counter not being shown so
    /// `None` should be used
    #[props(default)]
    pub char_counter: Option<TextAreaCharCounter>,
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
    /// Type: `number | string` so I'll leave it as a string
    #[props(into)]
    pub min: Option<String>,
    /// Type: `number | string`  so I'll leave it as a string
    #[props(into)]
    pub max: Option<String>,
    #[props(default)]
    pub size: Option<i64>, // --|
    #[props(default)] //   | -- What you doing step size
    pub step: Option<i64>, // --|
    #[props(default)]
    pub auto_validate: bool,
    #[props(default)]
    pub validity_transform: Option<ValidityTransform>,
    #[props(default)]
    pub validate_on_initial_render: bool,
    //#[props(into)]
    // the name cannot start with `on` or dioxus will expect an `EventHandler` which aren't static
    // and thus cannot be used here
    //pub _oninput: Option<StaticCallback<String>>,
    //_lifetime: Option<PhantomData<&'a ()>>,
    #[props(default)]
    pub name: Option<String>,

    #[props(into, default)]
    pub style: String,
    #[props(into, default)]
    pub class: String,
    #[props(into)]
    pub slot: Option<String>,
    #[props(default)]
    pub dialog_initial_focus: bool,
    #[props(default)]
    pub oninput: EventHandler<FormEvent>,
}

#[component]
pub fn MatTextArea(props: TextAreaProps) -> Element {
    let id = crate::use_id("textarea");
    //let input_listener = use_hook(|| None);
    //let validity_transform_closure = use_hook(|| None);
    if let Some(elem) = crate::get_elem_by_id(&id) {
        let target = elem.clone();
        let textarea = JsValue::from(elem).dyn_into::<TextArea>().unwrap();
        textarea.set_type(&JsValue::from(props.field_type.as_str()));
        textarea.set_value(&JsValue::from_str(
            props
                .value
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or_default(),
        ));
        /*if let Some(listener) = props._oninput.clone() {
            *input_listener = Some(set_on_input_handler(&target, listener, |(_, detail)| {
                detail
                    .unchecked_into::<MatTextAreaInputEvent>()
                    .target()
                    .value()
            }));
        }*/
        /*if let (Some(transform), None) = (
            props.validity_transform.clone(),
            &validity_transform_closure,
        ) {
            *validity_transform_closure = Some(Closure::wrap(Box::new(
                move |s: String, v: NativeValidityState| -> ValidityStateJS {
                    transform.0(s, v).into()
                },
            )
                as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
            textarea.set_validity_transform(validity_transform_closure.as_ref().unwrap());
        }*/
    }

    rsx! {
        mwc-textarea {
            id: id,

            //rows: props.rows.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap(),
            //cols: props.cols.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            label: props.label,
            placeholder: props.placeholder,
            icon: props.icon,
            iconTrailing: props.icon_trailing,
            disabled: props.disabled,
            //charCounter: props.char_counter.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            outlined: props.outlined,
            helper: props.helper,
            helperPersistent: props.helper_persistent,
            required: props.required,
            //maxLength: props.max_length.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            validationMessage: props.validation_message,
            min: props.min,
            max: props.max,
            //size: props.size.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            //step: props.step.map(|v| format_args!("{v}").into_value(cx.bump())).unwrap_or(AttributeValue::None),
            autoValidate: props.auto_validate,
            validateOnInitialRender: props.validate_on_initial_render,
            name: props.name,

            style: props.style,
            class: props.class,
            slot: props.slot,
            oninput: move |event| props.oninput.call(event),
            dialogInitialFocus: props.dialog_initial_focus,
        }
    }
}

//component!('a, MatTextArea, TextAreaProps, render, TextArea, "textarea");

#[wasm_bindgen]
extern "C" {
    type MatTextAreaInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextAreaInputEvent) -> TextArea;
}
