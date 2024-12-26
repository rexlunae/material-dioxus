use dioxus::prelude::*;
use std::fmt;

/// Dialog action type.
#[derive(Clone, PartialEq)]
pub enum ActionType {
    /// Binds `to slot` of `primaryAction`
    Primary,
    /// Binds `to slot` of `secondaryAction`
    Secondary,
}

impl ActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::Primary => "primaryAction",
            ActionType::Secondary => "secondaryAction",
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Props for [`MatDialogAction`]
#[derive(Clone, Props, PartialEq)]
pub struct ActionProps {
    pub action_type: ActionType,
    #[props(into)]
    pub action: Option<String>,
    pub children: Element,
}

/// Defines actions for [`MatDialog`][crate::MatDialog].
///
/// The passed children are wrapped in a `span` with the required attributes
/// set.
#[component]
pub fn MatDialogAction(props: ActionProps) -> Element {
    rsx! {
        span {
            slot: "{props.action_type}",
            direction: props.action,
            {props.children}
        }
    }
}
