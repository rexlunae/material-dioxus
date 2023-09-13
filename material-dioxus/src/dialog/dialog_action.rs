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
    fn as_str(&self) -> &'static str {
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
#[derive(Props)]
pub struct ActionProps<'a> {
    pub action_type: ActionType,
    #[props(into)]
    pub action: Option<String>,
    pub children: Element<'a>,
}

/// Defines actions for [`MatDialog`][crate::MatDialog].
///
/// The passed children are wrapped in a `span` with the required attributes
/// set.
#[allow(non_snake_case)]
pub fn MatDialogAction<'a>(cx: Scope<'a, ActionProps<'a>>) -> Element<'a> {
    render! {
        span {
            slot: "{cx.props.action_type}",
            "dialogAction": optional_string_attr!(cx.props.action),
            &cx.props.children
        }
    }
}
