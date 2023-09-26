# Material Dioxus

_Material Dioxus_ is a components library wrapper around Google's
[Material Web Components](https://github.com/material-components/material-components-web-components)
for the [Dioxus framework](https://dioxuslabs.com/). Only the `dioxus-web`
renderer is supported.

## Example

```rust
use material_dioxus::MatButton;
use dioxus::prelude::*;

rsx! {
    MatButton {
        label: "Click me!",
    }
};
```

## Getting started

### Installation

`Cargo.toml`:

```bash
cargo add material-dioxus --features full
```

Material icons and a Material font must be imported for full functionality.\
`Dioxus.toml`:

```toml
[web.resource]
style = [
    "https://fonts.googleapis.com/css?family=Roboto:300,400,500",
    "https://fonts.googleapis.com/css?family=Material+Icons&display=block",
    # ...
]
```

### Feature flags

There is one cargo feature for each component:

<!-- dprint-ignore-start -->
- `button`
- `circular-progress`
- `checkbox`
- `circular-progress-four-color`
<!-- - `drawer` -->
<!-- - `top-app-bar` -->
- `icon-button`
- `fab`
- `formfield`
<!-- - `linear-progress` -->
- `icon`
- `radio`
- `switch`
<!-- - `top-app-bar-fixed` -->
- `dialog`
- `list`
<!-- - `icon-button-toggle` -->
<!-- - `slider` -->
<!-- - `tabs` -->
<!-- - `snackbar` -->
- `textfield`
- `textarea`
<!-- - `select` -->
<!-- - `menu` -->
<!-- dprint-ignore-end -->

The `all-components` feature enables all components.

Additionally, there are two features related to theming.

- `theming` &emdash; Provides a `MatTheme` component for setting a color theme.
- `palette` &emdash; Provides constants for the material color palette
  (automatically enabled by `theming`).

The `full` feature enables all features.

## Theming

These components respect the theming applied to Material Web Components using
stylesheets.
[Learn about how to theme Material Web Components.](https://github.com/material-components/material-web/blob/mwc/docs/theming.md)

For convenience, the `theming` feature provides a `MatTheme` component, which
takes a few colors and sets all required CSS variables. Just include that in the
root of your application once.

## Event handling

Due to lifetime limitations of the normal Dioxus event handlers, material-dioxus
cannot make use of them. The exposed events instead use a custom callback type.
For simple buttons that are never disabled you can also just wrap them in a
`span` and use a normal event handler on that. For example

```rust
use dioxus::prelude::*;
use material_dioxus::MatButton;

#[allow(non_snake_case)]
fn Counter(cx: Scope) -> Element {
    let mut counter = use_state(cx, || 0);

    render! {
        // option 1: wrap the component in a span and use normal event handling
        span {
            onclick: move |_| counter += 1,
            MatButton { label: "click me: {counter}" }
        }
        // option 2: use the event handlers exposed by the component to respect
        // thinks like a button being disabled.
        // The closure must be 'static so we make use of `to_owned!`.
        MatButton {
            label: "click me: {counter}",
            _onclick: {
                to_owned![counter];
                move |_| counter += 1
            },
        }
    }
}
```

## Documentation

Full API documentation can be found [here](https://docs.rs/material-dioxus/).
Demos of components can be found [here](https://material-dioxus.rubixdev.de/).
