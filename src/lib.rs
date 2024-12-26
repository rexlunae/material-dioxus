// See: https://github.com/rustwasm/wasm-bindgen/issues/2774
// Can remove when wasm-bindgen is updated.
#![allow(clippy::unused_unit)]
#![doc = include_str!("../README.md")]

use dioxus::prelude::*;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
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

fn event_into_details(event: &web_sys::Event) -> JsValue {
    JsValue::from(event)
        .dyn_into::<web_sys::CustomEvent>()
        .unwrap_or_else(|_| panic!("could not convert to CustomEvent"))
        .detail()
}

fn event_details_into<T: JsCast>(event: &web_sys::Event) -> T {
    event_into_details(event).unchecked_into::<T>()
}

fn use_id(prefix: impl AsRef<str>) -> String {
    let prefix = prefix.as_ref();
    use_hook(|| {
        let mut id = format!("{prefix}-");
        Alphanumeric.append_string(&mut rand::thread_rng(), &mut id, 11);
        // rerender the component immediately, so that code depending on the ID works on "the
        // first render".
        id
    })
}

fn get_elem_by_id(id: impl AsRef<str>) -> Option<web_sys::Element> {
    let id = id.as_ref();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
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
pub use list::{MatCheckListItem, MatList, MatListItem, MatListSeparator, MatRadioListItem};

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
    //#[derive(Debug)]
    type Ripple;

    #[wasm_bindgen(getter, static_method_of = Ripple)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Ripple);
