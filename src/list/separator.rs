use dioxus::prelude::*;

/// Docs [here](https://github.com/material-components/material-web/tree/mwc/packages/list#dividers).
#[derive(Clone, Props, PartialEq)]
pub struct ListSeparatorProps {
    #[props(default)]
    pub padded: bool,
    #[props(default)]
    pub inset: bool,
}

#[component]
pub fn MatListSeparator(props: ListSeparatorProps) -> Element {
    rsx! {
        li {
            "divider": true,
            role: "separator",
            "padded": props.padded,
            "inset": props.inset,
        }
    }
}
