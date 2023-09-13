#![doc(html_root_url = "/docs")]
// See: https://github.com/rustwasm/wasm-bindgen/issues/2774
// Can remove when wasm-bindgen is updated.
#![allow(clippy::unused_unit)]
//! A Material components library for [Yew](https://yew.rs). It wrpas around [Material Web Components](https://github.com/material-components/material-web) exposing Yew components.
//!
//! Example usage:
//! ```rust
//! use material_yew::MatButton;
//! use yew::html;
//!
//! html! {
//!     <MatButton label="Click me!" />
//! };
//! ```
//!
//! All the main components from the modules are re-exported.
//! The specialized components used for populating slots and models can be
//! accessed from their respective modules.
//!
//! More information can be found on the [website](https://yew-material.web.app) and in the [GitHub README](https://github.com/hamza1311/yew-material)

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
mod utils;

// this macro is defined here so we can access it in the modules
macro_rules! loader_hack {
    ($ty:ty) => {
        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        impl $ty {
            #[allow(dead_code)]
            fn ensure_loaded() {
                LOADED.call_once(|| {
                    <$ty>::_dummy_loader();
                });
            }
        }
    };
}

macro_rules! component {
    ($comp: ident, $props: ty, $html: expr, $mwc_to_initialize: ident, $mwc_name: literal) => {
        paste::paste! {
            #[doc = "The `mwc-" $mwc_name "` component"]
            #[doc = ""]
            #[doc = "[MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/" $mwc_name ")"]
            #[allow(non_snake_case)]
            pub fn $comp(cx: Scope<$props>) -> Element {
                $mwc_to_initialize::ensure_loaded();
                $html(cx)
            }
        }
   };
    ('a, $comp: ident, $props: ty, $html: expr, $mwc_to_initialize: ident, $mwc_name: literal) => {
        paste::paste! {
            #[doc = "The `mwc-" $mwc_name "` component"]
            #[doc = ""]
            #[doc = "[MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/" $mwc_name ")"]
            #[allow(non_snake_case)]
            pub fn $comp<'a>(cx: Scope<'a, $props<'a>>) -> Element<'a> {
                $mwc_to_initialize::ensure_loaded();
                $html(cx)
            }
        }
   };
}

macro_rules! string_attr {
    ($value:expr) => {
        ::dioxus::core::AttributeValue::Text(&$value)
    }; /* (? $value:expr) => {{
        *     ::dioxus::core::AttributeValue::Text(&$value)
        * }}; */
}

macro_rules! optional_string_attr {
    ($value:expr) => {
        $value
            .as_ref()
            .map(|s| ::dioxus::core::AttributeValue::Text(s.as_str()))
            .unwrap_or(::dioxus::core::AttributeValue::None)
    };
}

macro_rules! bool_attr {
    ($value:expr) => {
        $value
            .then_some(::dioxus::core::AttributeValue::Bool(true))
            .unwrap_or(::dioxus::core::AttributeValue::None)
    };
}

fn event_into_details(event: &web_sys::Event) -> JsValue {
    JsValue::from(event)
        .dyn_into::<web_sys::CustomEvent>()
        .unwrap_or_else(|_| panic!("could not convert to CustomEvent"))
        .detail()
}
fn event_details_into<T: JsCast>(event: &web_sys::Event) -> T {
    event_into_details(event).unchecked_into::<T>()
}

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "button")]
#[doc(hidden)]
pub use button::MatButton;

#[cfg(feature = "circular-progress")]
pub mod circular_progress;
#[cfg(feature = "circular-progress")]
#[doc(hidden)]
pub use circular_progress::MatCircularProgress;

#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "checkbox")]
#[doc(hidden)]
pub use checkbox::MatCheckbox;

#[cfg(feature = "circular-progress-four-color")]
pub mod circular_progress_four_color;
#[cfg(feature = "circular-progress-four-color")]
#[doc(hidden)]
pub use circular_progress_four_color::MatCircularProgressFourColor;

#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "drawer")]
#[doc(hidden)]
pub use drawer::MatDrawer;

#[cfg(feature = "top-app-bar")]
pub mod top_app_bar;
#[cfg(feature = "top-app-bar")]
#[doc(hidden)]
pub use top_app_bar::MatTopAppBar;

#[cfg(feature = "icon-button")]
pub mod icon_button;
#[cfg(feature = "icon-button")]
#[doc(hidden)]
pub use icon_button::MatIconButton;

#[cfg(feature = "fab")]
pub mod fab;
#[cfg(feature = "fab")]
#[doc(hidden)]
pub use fab::MatFab;

#[cfg(feature = "formfield")]
pub mod form_field;
#[cfg(feature = "formfield")]
#[doc(hidden)]
pub use form_field::MatFormfield;

#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "icon")]
#[doc(hidden)]
pub use icon::MatIcon;

#[cfg(feature = "linear-progress")]
pub mod linear_progress;
#[cfg(feature = "linear-progress")]
#[doc(hidden)]
pub use linear_progress::MatLinearProgress;

#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "radio")]
#[doc(hidden)]
pub use radio::MatRadio;

#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
#[doc(hidden)]
pub use switch::MatSwitch;

#[cfg(feature = "top-app-bar-fixed")]
pub mod top_app_bar_fixed;
#[cfg(feature = "top-app-bar-fixed")]
#[doc(hidden)]
pub use top_app_bar_fixed::MatTopAppBarFixed;

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dialog")]
#[doc(hidden)]
pub use dialog::MatDialog;

#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "list")]
#[doc(no_inline)]
#[doc(hidden)]
pub use list::{MatCheckListItem, MatList, MatListItem, MatRadioListItem};

#[cfg(feature = "icon-button-toggle")]
pub mod icon_button_toggle;
#[cfg(feature = "icon-button-toggle")]
#[doc(hidden)]
pub use icon_button_toggle::MatIconButtonToggle;

#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "slider")]
#[doc(hidden)]
pub use slider::MatSlider;

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
#[doc(no_inline)]
#[doc(hidden)]
pub use tabs::{MatTab, MatTabBar};

#[cfg(feature = "snackbar")]
pub mod snackbar;
#[cfg(feature = "snackbar")]
#[doc(hidden)]
pub use snackbar::MatSnackbar;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod text_inputs;
#[cfg(feature = "textarea")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::MatTextArea;
#[cfg(feature = "textfield")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::MatTextField;

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "select")]
#[doc(hidden)]
pub use select::MatSelect;

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menu")]
#[doc(hidden)]
pub use menu::MatMenu;

#[cfg(feature = "theming")]
pub mod theming;
#[cfg(feature = "theming")]
#[doc(hidden)]
pub use theming::MatTheme;

#[cfg(feature = "palette")]
pub mod palette;

pub use utils::StaticCallback;

#[wasm_bindgen(module = "/build/core.js")]
extern "C" {
    #[derive(Debug)]
    type Ripple;

    #[wasm_bindgen(getter, static_method_of = Ripple)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Ripple);
