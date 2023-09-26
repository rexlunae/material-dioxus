use dioxus::prelude::*;

/// Docs [here](https://github.com/material-components/material-web/tree/mwc/packages/list#dividers).
#[derive(Props, PartialEq)]
pub struct ListSeparatorProps {
    #[props(default)]
    pub padded: bool,
    #[props(default)]
    pub inset: bool,
}

#[allow(non_snake_case)]
pub fn MatListSeparator(cx: Scope<ListSeparatorProps>) -> Element {
    render! {
        li {
            "divider": true,
            role: "separator",
            "padded": bool_attr!(cx.props.padded),
            "inset": bool_attr!(cx.props.inset),
        }
    }
}
