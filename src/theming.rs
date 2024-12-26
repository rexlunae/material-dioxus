use std::fmt;

use ::palette::{blend::Blend, color_difference::Wcag21RelativeContrast, Mix, Srgb, Srgba};
use dioxus::prelude::*;

use crate::palette::{self, Color};

#[derive(Clone, Debug, PartialEq)]
pub struct Colors {
    pub primary: Color,
    pub secondary: Option<Color>,
    pub surface: Option<Color>,
    pub inverse_surface: Color,
    pub background: Color,
    pub error: Color,

    pub on_primary: Option<Color>,
    pub on_secondary: Option<Color>,
    pub on_surface: Option<Color>,
    pub on_inverse_surface: Option<Color>,
    pub on_error: Option<Color>,

    pub four_color_progress: [Color; 4],
    pub switch_use_secondary: bool,
}

impl Colors {
    pub const DEFAULT_LIGHT: Self = Self {
        primary: palette::PURPLE_500,
        secondary: None,
        surface: None,
        inverse_surface: palette::from_u32(0x121212, 1.),
        background: palette::from_u32(0xffffff, 1.),
        error: palette::from_u32(0xb00020, 1.),

        on_primary: None,
        on_secondary: None,
        on_surface: None,
        on_inverse_surface: None,
        on_error: None,

        four_color_progress: [
            palette::GREEN_500,
            palette::YELLOW_500,
            palette::RED_500,
            palette::BLUE_500,
        ],
        switch_use_secondary: false,
    };
    pub const DEFAULT_DARK: Self = Self {
        primary: palette::PURPLE_200,
        inverse_surface: palette::from_u32(0xffffff, 1.),
        background: palette::from_u32(0x121212, 1.),
        error: palette::from_u32(0xcf6679, 1.),
        ..Self::DEFAULT_LIGHT
    };
}

#[derive(Clone, Props, PartialEq)]
pub struct ThemeProps {
    #[props(default = Colors::DEFAULT_LIGHT)]
    theme: Colors,
    #[props(!optional, default = Some(Colors::DEFAULT_DARK))]
    dark_theme: Option<Colors>,
}

#[derive(Clone, Copy)]
struct ColorDisplay(Color);
impl fmt::Display for ColorDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b, a) = self.0.into_components();
        write!(f, "rgba({r}, {g}, {b}, {a})")
    }
}

fn overlay(base: Color, overlay: Color, alpha: f32) -> Color {
    let base: Srgba = base.into_format();
    base.overlay(palette::with_alpha(overlay, alpha).into_format())
        .into_format()
}

pub fn contrast_text(bg: Color) -> Color {
    let bg = Srgba::<f32>::from_format(bg);
    let light_contrast = bg.relative_contrast(Srgb::new(1., 1., 1.));
    let dark_contrast = bg.relative_contrast(Srgb::new(0., 0., 0.));
    const MIN_CONTRAST: f32 = 3.1;
    if light_contrast < MIN_CONTRAST && dark_contrast > light_contrast {
        palette::from_u32(0x000000, 0.87)
    } else {
        palette::from_u32(0xffffff, 1.)
    }
}

fn mix(col1: Color, col2: Color, factor: f32) -> Color {
    let col1 = Srgba::<f32>::from_format(col1);
    let col2 = Srgba::<f32>::from_format(col2);
    col2.mix(col1, factor).into_format()
}

#[rustfmt::skip]
fn define_vars(colors: &Colors) -> String {
    let primary = ColorDisplay(colors.primary);
    let secondary = ColorDisplay(colors.secondary.unwrap_or(colors.primary));
    let surface = ColorDisplay(colors.surface.unwrap_or(colors.background));
    let background = ColorDisplay(colors.background);
    let error = ColorDisplay(colors.error);

    let on_primary = ColorDisplay(colors.on_primary.unwrap_or(contrast_text(primary.0)));
    let on_secondary = ColorDisplay(colors.on_secondary.unwrap_or(contrast_text(secondary.0)));
    let on_surface = ColorDisplay(colors.on_surface.unwrap_or(contrast_text(surface.0)));
    let on_inverse_surface = ColorDisplay(colors.on_inverse_surface.unwrap_or(contrast_text(colors.inverse_surface)));
    let on_error = ColorDisplay(colors.on_error.unwrap_or(contrast_text(colors.error)));

    let surface_alpha = |alpha: f32| palette::with_alpha(on_surface.0, alpha);
    let inverse_surface_alpha = |alpha: f32| palette::with_alpha(on_inverse_surface.0, alpha);

    let text_primary_on_bg = ColorDisplay(surface_alpha(0.87));
    let text_secondary_on_bg = ColorDisplay(surface_alpha(0.6));
    let text_hint_on_bg = ColorDisplay(surface_alpha(0.6));
    let text_disabled_on_bg = ColorDisplay(surface_alpha(0.38));
    let text_icon_on_bg = ColorDisplay(surface_alpha(0.6));
    let text_primary_on_inverse = ColorDisplay(inverse_surface_alpha(0.87));
    let text_secondary_on_inverse = ColorDisplay(inverse_surface_alpha(0.6));
    let text_hint_on_inverse = ColorDisplay(inverse_surface_alpha(0.6));
    let text_disabled_on_inverse = ColorDisplay(inverse_surface_alpha(0.38));
    let text_icon_on_inverse = ColorDisplay(inverse_surface_alpha(0.6));

    let button_outline = ColorDisplay(surface_alpha(0.12));
    let button_disabled_fill = ColorDisplay(surface_alpha(0.12));
    let button_disabled_ink = ColorDisplay(surface_alpha(0.37));
    let button_disabled_outline = ColorDisplay(surface_alpha(0.12));

    let four_color_progress_1 = ColorDisplay(colors.four_color_progress[0]);
    let four_color_progress_2 = ColorDisplay(colors.four_color_progress[1]);
    let four_color_progress_3 = ColorDisplay(colors.four_color_progress[2]);
    let four_color_progress_4 = ColorDisplay(colors.four_color_progress[3]);

    let checkbox_ink = ColorDisplay(on_secondary.0);
    let checkbox_unchecked = ColorDisplay(surface_alpha(0.54));
    let checkbox_disabled = ColorDisplay(surface_alpha(0.38));

    // TODO: disabled switch on dark is barely visible
    let switch_primary = ColorDisplay(if colors.switch_use_secondary { secondary.0 } else { colors.primary });
    let switch_on_surface = ColorDisplay(palette::GREY_800);
    let switch_on_surface_state_content = ColorDisplay(palette::GREY_700);
    let switch_hairline = ColorDisplay(palette::GREY_400);
    let switch_primary_state_content = switch_primary;
    let switch_inverse_primary = ColorDisplay(overlay(switch_hairline.0, switch_primary.0, 0.25));

    let textfield_idle_line = ColorDisplay(surface_alpha(0.42));
    let textfield_hover_line = ColorDisplay(surface_alpha(0.87));
    let textfield_disabled_line = ColorDisplay(surface_alpha(0.06));
    let textfield_idle_border = ColorDisplay(surface_alpha(0.38));
    let textfield_hover_border = ColorDisplay(surface_alpha(0.87));
    let textfield_disabled_border = ColorDisplay(surface_alpha(0.06));
    let textfield_fill = ColorDisplay(mix(on_surface.0, surface.0, 0.04));
    let textfield_disabled_fill = ColorDisplay(mix(on_surface.0, surface.0, 0.02));
    let textfield_ink = ColorDisplay(surface_alpha(0.87));
    let textfield_label_ink = ColorDisplay(surface_alpha(0.6));
    let textfield_disabled_ink = ColorDisplay(surface_alpha(0.37));
    let textfield_icon = ColorDisplay(surface_alpha(0.54));
    let textfield_disabled_icon = ColorDisplay(surface_alpha(0.3));

    let dialog_scrim = ColorDisplay(surface_alpha(0.32));
    let dialog_heading = ColorDisplay(surface_alpha(0.87));
    let dialog_content = ColorDisplay(surface_alpha(0.6));
    let dialog_divider = ColorDisplay(surface_alpha(0.12));

    let textarea_idle_border = ColorDisplay(surface_alpha(0.38));
    let textarea_hover_border = ColorDisplay(surface_alpha(0.87));
    let textarea_disabled_border = ColorDisplay(surface_alpha(0.06));

    let list_ripple = ColorDisplay(surface_alpha(1.));
    let list_divider = ColorDisplay(surface_alpha(0.12));

    format!(
        "
:root {{
    --mdc-theme-primary: {primary};
    --mdc-theme-secondary: {secondary};
    --mdc-theme-surface: {surface};
    --mdc-theme-background: {background};
    --mdc-theme-error: {error};

    --mdc-theme-on-primary: {on_primary};
    --mdc-theme-on-secondary: {on_secondary};
    --mdc-theme-on-surface: {on_surface};
    --mdc-theme-on-error: {on_error};

    --mdc-theme-text-primary-on-background: {text_primary_on_bg};
    --mdc-theme-text-secondary-on-background: {text_secondary_on_bg};
    --mdc-theme-text-hint-on-background: {text_hint_on_bg};
    --mdc-theme-text-disabled-on-background: {text_disabled_on_bg};
    --mdc-theme-text-icon-on-background: {text_icon_on_bg};
    --mdc-theme-text-primary-on-light: {text_primary_on_bg};
    --mdc-theme-text-secondary-on-light: {text_secondary_on_bg};
    --mdc-theme-text-hint-on-light: {text_hint_on_bg};
    --mdc-theme-text-disabled-on-light: {text_disabled_on_bg};
    --mdc-theme-text-icon-on-light: {text_icon_on_bg};
    --mdc-theme-text-primary-on-dark: {text_primary_on_inverse};
    --mdc-theme-text-secondary-on-dark: {text_secondary_on_inverse};
    --mdc-theme-text-hint-on-dark: {text_hint_on_inverse};
    --mdc-theme-text-disabled-on-dark: {text_disabled_on_inverse};
    --mdc-theme-text-icon-on-dark: {text_icon_on_inverse};

    --mdc-button-outline-color: {button_outline};
    --mdc-button-disabled-fill-color: {button_disabled_fill};
    --mdc-button-disabled-ink-color: {button_disabled_ink};
    --mdc-button-disabled-outline-color: {button_disabled_outline};

    --mdc-circular-progress-bar-color-1: {four_color_progress_1};
    --mdc-circular-progress-bar-color-2: {four_color_progress_2};
    --mdc-circular-progress-bar-color-3: {four_color_progress_3};
    --mdc-circular-progress-bar-color-4: {four_color_progress_4};

    --mdc-checkbox-ink-color: {checkbox_ink};
    --mdc-checkbox-unchecked-color: {checkbox_unchecked};
    --mdc-checkbox-disabled-color: {checkbox_disabled};

    --mdc-radio-unchecked-color: {checkbox_unchecked};
    --mdc-radio-disabled-color: {checkbox_disabled};

    --mdc-switch-disabled-selected-handle-color: {switch_on_surface};
    --mdc-switch-disabled-selected-track-color: {switch_on_surface};
    --mdc-switch-disabled-unselected-handle-color: {switch_on_surface};
    --mdc-switch-disabled-unselected-track-color: {switch_on_surface};
    --mdc-switch-selected-focus-handle-color: {switch_primary_state_content};
    --mdc-switch-selected-focus-track-color: {switch_inverse_primary};
    --mdc-switch-selected-hover-handle-color: {switch_primary_state_content};
    --mdc-switch-selected-hover-track-color: {switch_inverse_primary};
    --mdc-switch-selected-pressed-handle-color: {switch_primary_state_content};
    --mdc-switch-selected-pressed-track-color: {switch_inverse_primary};
    --mdc-switch-selected-track-color: {switch_inverse_primary};
    --mdc-switch-unselected-focus-handle-color: {switch_on_surface_state_content};
    --mdc-switch-unselected-focus-state-layer-color: {switch_on_surface};
    --mdc-switch-unselected-focus-track-color: {switch_hairline};
    --mdc-switch-unselected-handle-color: {switch_on_surface_state_content};
    --mdc-switch-unselected-hover-handle-color: {switch_on_surface_state_content};
    --mdc-switch-unselected-hover-state-layer-color: {switch_on_surface};
    --mdc-switch-unselected-hover-track-color: {switch_hairline};
    --mdc-switch-unselected-pressed-handle-color: {switch_on_surface_state_content};
    --mdc-switch-unselected-pressed-state-layer-color: {switch_on_surface};
    --mdc-switch-unselected-pressed-track-color: {switch_hairline};
    --mdc-switch-unselected-track-color: {switch_hairline};

    --mdc-text-field-idle-line-color: {textfield_idle_line};
    --mdc-text-field-hover-line-color: {textfield_hover_line};
    --mdc-text-field-disabled-line-color: {textfield_disabled_line};
    --mdc-text-field-outlined-idle-border-color: {textfield_idle_border};
    --mdc-text-field-outlined-hover-border-color: {textfield_hover_border};
    --mdc-text-field-outlined-disabled-border-color: {textfield_disabled_border};
    --mdc-text-field-fill-color: {textfield_fill};
    --mdc-text-field-disabled-fill-color: {textfield_disabled_fill};
    --mdc-text-field-ink-color: {textfield_ink};
    --mdc-text-field-label-ink-color: {textfield_label_ink};
    --mdc-text-field-disabled-ink-color: {textfield_disabled_ink};
    --mdc-text-field-icon-color: {textfield_icon};
    --mdc-text-field-disabled-icon-color: {textfield_disabled_icon};

    --mdc-dialog-scrim-color: {dialog_scrim};
    --mdc-dialog-heading-ink-color: {dialog_heading};
    --mdc-dialog-content-ink-color: {dialog_content};
    --mdc-dialog-scroll-divider-color: {dialog_divider};

    --mdc-text-area-outlined-idle-border-color: {textarea_idle_border};
    --mdc-text-area-outlined-hover-border-color: {textarea_hover_border};
    --mdc-text-area-outlined-disabled-border-color: {textarea_disabled_border};

    --mdc-deprecated-list-divider-color: {list_divider};
}}

mwc-switch {{
    --mdc-theme-primary: {switch_primary};
}}

mwc-list {{
    --mdc-ripple-color: {list_ripple};
}}
"
    )
}

#[component]
pub fn MatTheme(props: ThemeProps) -> Element {
    let light_theme = define_vars(&props.theme);
    let dark_theme = if let Some(colors) = &props.dark_theme {
        format!(
            "@media screen and (prefers-color-scheme: dark) {{{}}}",
            define_vars(colors)
        )
    } else {
        String::new()
    };

    rsx! {
        style { dangerous_inner_html: "{light_theme}{dark_theme}" }
    }
}
