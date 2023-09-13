use dioxus::prelude::*;
use material_dioxus::{
    dialog::{ActionType, MatDialogAction},
    text_inputs::{TextFieldType, ValidityState, ValidityTransform},
    MatButton, MatCheckbox, MatCircularProgress, MatCircularProgressFourColor, MatDialog, MatFab,
    MatFormfield, MatIcon, MatIconButton, MatRadio, MatSwitch, MatTextField, MatTheme,
};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let circular_progress_closed = use_state(cx, || false);
    let mut circular_progress_progress = use_state(cx, || 0.2);
    let cb_value = use_state(cx, || true);
    let switch_value = use_state(cx, || true);
    let textfield_value = use_state(cx, String::new);
    let basic_dialog_open = use_state(cx, || false);
    let scrollable_dialog_open = use_state(cx, || false);

    render! {
        style {
            dangerous_inner_html: "
.demo {{
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    border: 1px solid currentColor;
    padding: 1rem;
    margin: 1rem;
}}

body {{
    background-color: var(--mdc-theme-background);
}}

html {{
    color-scheme: light dark;
}}
"
        }
        MatTheme {}

        div {
            class: "demo",
            span {
                onclick: move |_| basic_dialog_open.set(true),
                MatButton { raised: true, label: "basic" }
            }
            MatDialog {
                heading: "Dialog Heading",
                open: **basic_dialog_open,
                _onclosed: {
                    to_owned![basic_dialog_open];
                    move |action| {
                        gloo_console::log!(action);
                        basic_dialog_open.set(false);
                    }
                },
                div { "Dialog body text" }
                MatTextField { label: "I am auto-focused", dialog_initial_focus: true }
                MatTextField { label: "I am not auto-focused" }
                MatDialogAction {
                    action_type: ActionType::Primary,
                    action: "ok",
                    MatButton { label: "ok" }
                }
                MatDialogAction {
                    action_type: ActionType::Secondary,
                    action: "cancel",
                    MatButton { label: "cancel" }
                }
                MatDialogAction {
                    action_type: ActionType::Secondary,
                    MatButton { label: "other" }
                }
            }

            span {
                onclick: move |_| scrollable_dialog_open.set(true),
                MatButton { raised: true, label: "scrollable" }
            }
            MatDialog {
                heading: "Scrollable",
                stacked: true,
                open: **scrollable_dialog_open,
                _onclosed: {
                    to_owned![scrollable_dialog_open];
                    move |_| scrollable_dialog_open.set(false)
                },
                div { ("Really long text will scroll. ").repeat(100) }
                MatDialogAction {
                    action_type: ActionType::Primary,
                    action: "primary",
                    MatButton { label: "primary" }
                }
                MatDialogAction {
                    action_type: ActionType::Secondary,
                    action: "secondary",
                    MatButton { label: "secondary" }
                }
            }
        }

        div {
            class: "demo",
            MatIcon { "sentiment_very_dissatisfied" }
            MatIcon { "sentiment_dissatisfied" }
            MatIcon { "sentiment_neutral" }
            MatIcon { "sentiment_satisfied_alt" }
            MatIcon { "sentiment_very_satisfied" }
            MatIcon {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: 24,
                    height: 24,
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    path {
                        d: "M0 0h24v24H0z",
                        fill: "none",
                    }
                    path {
                        d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
                    }
                }
            }
        }

        div {
            class: "demo",
            MatTextField {
                label: "Standard",
            }
            MatTextField { label: "Standard", icon: "event", field_type: TextFieldType::Date }
            MatTextField { label: "Standard", icon_trailing: "delete" }

            MatTextField {
                label: "Outlined",
                outlined: true,
                value: "{textfield_value}",
                _oninput: {
                    to_owned![textfield_value];
                    move |new_value| textfield_value.set(new_value)
                },
            }
            MatTextField { label: "Outlined", outlined: true, icon: "event", field_type: TextFieldType::Time }
            MatTextField { label: "Outlined", outlined: true, icon_trailing: "delete" }
            span { "value: {textfield_value}" }

            MatTextField {
                label: "What's the answer?",
                validity_transform: ValidityTransform::new(move |val, _| {
                    let mut state = ValidityState::new();
                    if val == "42" {
                        state.set_valid(true);
                    } else {
                        state.set_valid(false).set_bad_input(true);
                    }
                    state
                }),
                helper: "only \"42\" is valid",
                max_length: 10,
                char_counter: true,
            }
            MatTextField {
                label: "Persistant helper",
                helper: "I am helping",
                helper_persistent: true,
                placeholder: "placeholder",
                prefix: "prefix",
                suffix: "suffix",
            }
            MatTextField { label: "Disabled", disabled: true }
        }

        div {
            class: "demo",
            MatFormfield { label: "Checkbox", MatCheckbox {} }
            MatFormfield { label: "Align End", MatCheckbox {} }
            MatFormfield { label: "Switch", MatSwitch {} }
            MatFormfield { label: "Radio 1", MatRadio { name: "c" } }
            MatFormfield { label: "Radio 2", MatRadio { name: "c" } }
            MatFormfield { label: "Radio 3", MatRadio { name: "c", checked: true, disabled: true } }
        }

        div {
            class: "demo",
            MatFab { icon: "edit" }
            MatFab { icon: "add", mini: true }
            MatFab { icon: "add", mini: true, reduced_touch_target: true }
            MatFab {
                svg {
                    "slot": "icon",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: 24,
                    height: 24,
                    view_box: "0 0 24 24",
                    path {
                        d: "M0 0h24v24H0z",
                        fill: "none",
                    }
                    path {
                        d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
                    }
                }
            }
            MatFab { icon: "shopping_cart", label: "add to cart", extended: true }
            MatFab { icon: "shopping_cart", label: "add to cart", extended: true, show_icon_at_end: true }
        }

        div {
            class: "demo",
            MatIconButton { icon: "code" }
            MatIconButton { icon: "code", disabled: true }
            MatIconButton {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: 24,
                    height: 24,
                    view_box: "0 0 24 24",
                    path {
                        d: "M0 0h24v24H0z",
                        fill: "none",
                    }
                    path {
                        d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
                    }
                }
            }
        }

        div {
            class: "demo",
            MatSwitch {}
            MatSwitch { selected: true }
            MatSwitch { disabled: true, selected: **switch_value }
            span {
                onclick: move |_| switch_value.set(!switch_value),
                MatSwitch { selected: **switch_value }
            }
            span { "selected: {switch_value}" }
        }

        div {
            class: "demo",
            MatRadio { name: "a", checked: true }
            MatRadio { name: "a" }
            MatRadio { name: "a" }
            MatRadio { name: "b", checked: true }
            MatRadio { name: "b" }
            MatRadio { name: "b" }
            MatRadio { disabled: true, checked: true }
            MatRadio { disabled: true }
        }

        div {
            class: "demo",
            MatCheckbox {}
            MatCheckbox { checked: true }
            MatCheckbox { indeterminate: true }
            MatCheckbox { disabled: true }
            MatCheckbox { reduced_touch_target: true }
            MatCheckbox {
                checked: **cb_value,
                _onchange: {
                    to_owned![cb_value];
                    move |new_value| cb_value.set(new_value)
                },
            }
            span { "checked: {cb_value}" }
        }

        div {
            class: "demo",

            span {
                onclick: move |_| circular_progress_closed.set(!circular_progress_closed),
                MatButton {
                    label: "toggle",
                    raised: true,
                }
            }
            span {
                onclick: move |_| circular_progress_progress += 0.1,
                MatButton {
                    label: "increase progress",
                    raised: true,
                }
            }
            MatCircularProgress {
                closed: **circular_progress_closed,
                progress: **circular_progress_progress,
            }
            MatCircularProgress {
                closed: **circular_progress_closed,
                indeterminate: true,
            }
            MatCircularProgressFourColor {
                closed: **circular_progress_closed,
                indeterminate: true,
            }
        }

        div {
            class: "demo",

            MatButton { label: "test" }
            MatButton { label: "test", icon: "code" }

            MatButton { label: "test", outlined: true }
            MatButton { label: "test", icon: "code", outlined: true }

            MatButton { label: "test", raised: true }
            MatButton { label: "test", icon: "code", raised: true }

            MatButton { label: "test", unelevated: true }
            MatButton { label: "test", icon: "code", unelevated: true }

            MatButton { label: "test", dense: true }
            MatButton { label: "test", icon: "code", dense: true }

            MatButton { label: "test", icon: "code", trailing_icon: true }
            MatButton { label: "test", icon: "code", trailing_icon: true, outlined: true }
            MatButton { label: "test", icon: "code", trailing_icon: true, raised: true }
            MatButton { label: "test", icon: "code", trailing_icon: true, unelevated: true }
            MatButton { label: "test", icon: "code", trailing_icon: true, dense: true }

            MatButton { label: "test", icon: "code", disabled: true }
            MatButton { label: "test", icon: "code", disabled: true, outlined: true }
            MatButton { label: "test", icon: "code", disabled: true, raised: true }
            MatButton { label: "test", icon: "code", disabled: true, unelevated: true }
            MatButton { label: "test", icon: "code", disabled: true, dense: true }
        }
    }
}
