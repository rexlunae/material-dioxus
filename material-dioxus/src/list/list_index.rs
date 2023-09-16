use std::collections::HashSet;
use wasm_bindgen::{JsCast, JsValue};

/// The `MWCListIndex` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-1)
#[derive(Debug, Clone)]
pub enum ListIndex {
    /// Provided when `multi` prop is set to `false` (default) on the component
    ///
    /// `None` denotes value of `-1`
    Single(Option<usize>),
    /// Provided when `multi` prop is set to `true` on the component
    Multi(HashSet<usize>),
}

impl ListIndex {
    pub fn unwrap_single(self) -> Option<usize> {
        match self {
            ListIndex::Single(val) => val,
            ListIndex::Multi(_) => panic!("called `unwrap_single` on {self:?}"),
        }
    }

    pub fn unwrap_multi(self) -> HashSet<usize> {
        match self {
            ListIndex::Multi(val) => val,
            ListIndex::Single(_) => panic!("called `unwrap_multi` on {self:?}"),
        }
    }
}

impl From<JsValue> for ListIndex {
    fn from(val: JsValue) -> Self {
        if let Ok(set) = val.clone().dyn_into::<js_sys::Set>() {
            let indices = set
                .values()
                .into_iter()
                .filter_map(|item| item.ok())
                .filter_map(|value| value.as_f64())
                .map(|num| num as usize)
                .collect();
            ListIndex::Multi(indices)
        } else if let Some(value) = val.as_f64() {
            ListIndex::Single(if value != -1.0 {
                Some(value as usize)
            } else {
                None
            })
        } else {
            panic!("This should never happen")
        }
    }
}
