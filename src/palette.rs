//! Constants color definitions of the material color palette.
//!
//! See <https://github.com/material-components/material-web/blob/v0.27.0/components/compat/theme/_color-palette.scss>
//! and <https://m2.material.io/design/color/the-color-system.html#tools-for-picking-colors>.

use palette::{Alpha, Srgb};

pub type Color = Alpha<Srgb<u8>, f32>;

pub const fn from_u32(color: u32, alpha: f32) -> Color {
    let [_, r, g, b] = color.to_be_bytes();
    Color::new(r, g, b, alpha)
}

pub const fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { alpha, ..color }
}

pub const RED_50: Color = from_u32(0xffebee, 1.);
pub const RED_100: Color = from_u32(0xffcdd2, 1.);
pub const RED_200: Color = from_u32(0xef9a9a, 1.);
pub const RED_300: Color = from_u32(0xe57373, 1.);
pub const RED_400: Color = from_u32(0xef5350, 1.);
pub const RED_500: Color = from_u32(0xf44336, 1.);
pub const RED_600: Color = from_u32(0xe53935, 1.);
pub const RED_700: Color = from_u32(0xd32f2f, 1.);
pub const RED_800: Color = from_u32(0xc62828, 1.);
pub const RED_900: Color = from_u32(0xb71c1c, 1.);
pub const RED_A100: Color = from_u32(0xff8a80, 1.);
pub const RED_A200: Color = from_u32(0xff5252, 1.);
pub const RED_A400: Color = from_u32(0xff1744, 1.);
pub const RED_A700: Color = from_u32(0xd50000, 1.);

pub const PINK_50: Color = from_u32(0xfce4ec, 1.);
pub const PINK_100: Color = from_u32(0xf8bbd0, 1.);
pub const PINK_200: Color = from_u32(0xf48fb1, 1.);
pub const PINK_300: Color = from_u32(0xf06292, 1.);
pub const PINK_400: Color = from_u32(0xec407a, 1.);
pub const PINK_500: Color = from_u32(0xe91e63, 1.);
pub const PINK_600: Color = from_u32(0xd81b60, 1.);
pub const PINK_700: Color = from_u32(0xc2185b, 1.);
pub const PINK_800: Color = from_u32(0xad1457, 1.);
pub const PINK_900: Color = from_u32(0x880e4f, 1.);
pub const PINK_A100: Color = from_u32(0xff80ab, 1.);
pub const PINK_A200: Color = from_u32(0xff4081, 1.);
pub const PINK_A400: Color = from_u32(0xf50057, 1.);
pub const PINK_A700: Color = from_u32(0xc51162, 1.);

pub const PURPLE_50: Color = from_u32(0xf3e5f5, 1.);
pub const PURPLE_100: Color = from_u32(0xe1bee7, 1.);
pub const PURPLE_200: Color = from_u32(0xce93d8, 1.);
pub const PURPLE_300: Color = from_u32(0xba68c8, 1.);
pub const PURPLE_400: Color = from_u32(0xab47bc, 1.);
pub const PURPLE_500: Color = from_u32(0x9c27b0, 1.);
pub const PURPLE_600: Color = from_u32(0x8e24aa, 1.);
pub const PURPLE_700: Color = from_u32(0x7b1fa2, 1.);
pub const PURPLE_800: Color = from_u32(0x6a1b9a, 1.);
pub const PURPLE_900: Color = from_u32(0x4a148c, 1.);
pub const PURPLE_A100: Color = from_u32(0xea80fc, 1.);
pub const PURPLE_A200: Color = from_u32(0xe040fb, 1.);
pub const PURPLE_A400: Color = from_u32(0xd500f9, 1.);
pub const PURPLE_A700: Color = from_u32(0xaa00ff, 1.);

pub const DEEP_PURPLE_50: Color = from_u32(0xede7f6, 1.);
pub const DEEP_PURPLE_100: Color = from_u32(0xd1c4e9, 1.);
pub const DEEP_PURPLE_200: Color = from_u32(0xb39ddb, 1.);
pub const DEEP_PURPLE_300: Color = from_u32(0x9575cd, 1.);
pub const DEEP_PURPLE_400: Color = from_u32(0x7e57c2, 1.);
pub const DEEP_PURPLE_500: Color = from_u32(0x673ab7, 1.);
pub const DEEP_PURPLE_600: Color = from_u32(0x5e35b1, 1.);
pub const DEEP_PURPLE_700: Color = from_u32(0x512da8, 1.);
pub const DEEP_PURPLE_800: Color = from_u32(0x4527a0, 1.);
pub const DEEP_PURPLE_900: Color = from_u32(0x311b92, 1.);
pub const DEEP_PURPLE_A100: Color = from_u32(0xb388ff, 1.);
pub const DEEP_PURPLE_A200: Color = from_u32(0x7c4dff, 1.);
pub const DEEP_PURPLE_A400: Color = from_u32(0x651fff, 1.);
pub const DEEP_PURPLE_A700: Color = from_u32(0x6200ea, 1.);

pub const INDIGO_50: Color = from_u32(0xe8eaf6, 1.);
pub const INDIGO_100: Color = from_u32(0xc5cae9, 1.);
pub const INDIGO_200: Color = from_u32(0x9fa8da, 1.);
pub const INDIGO_300: Color = from_u32(0x7986cb, 1.);
pub const INDIGO_400: Color = from_u32(0x5c6bc0, 1.);
pub const INDIGO_500: Color = from_u32(0x3f51b5, 1.);
pub const INDIGO_600: Color = from_u32(0x3949ab, 1.);
pub const INDIGO_700: Color = from_u32(0x303f9f, 1.);
pub const INDIGO_800: Color = from_u32(0x283593, 1.);
pub const INDIGO_900: Color = from_u32(0x1a237e, 1.);
pub const INDIGO_A100: Color = from_u32(0x8c9eff, 1.);
pub const INDIGO_A200: Color = from_u32(0x536dfe, 1.);
pub const INDIGO_A400: Color = from_u32(0x3d5afe, 1.);
pub const INDIGO_A700: Color = from_u32(0x304ffe, 1.);

pub const BLUE_50: Color = from_u32(0xe3f2fd, 1.);
pub const BLUE_100: Color = from_u32(0xbbdefb, 1.);
pub const BLUE_200: Color = from_u32(0x90caf9, 1.);
pub const BLUE_300: Color = from_u32(0x64b5f6, 1.);
pub const BLUE_400: Color = from_u32(0x42a5f5, 1.);
pub const BLUE_500: Color = from_u32(0x2196f3, 1.);
pub const BLUE_600: Color = from_u32(0x1e88e5, 1.);
pub const BLUE_700: Color = from_u32(0x1976d2, 1.);
pub const BLUE_800: Color = from_u32(0x1565c0, 1.);
pub const BLUE_900: Color = from_u32(0x0d47a1, 1.);
pub const BLUE_A100: Color = from_u32(0x82b1ff, 1.);
pub const BLUE_A200: Color = from_u32(0x448aff, 1.);
pub const BLUE_A400: Color = from_u32(0x2979ff, 1.);
pub const BLUE_A700: Color = from_u32(0x2962ff, 1.);

pub const LIGHT_BLUE_50: Color = from_u32(0xe1f5fe, 1.);
pub const LIGHT_BLUE_100: Color = from_u32(0xb3e5fc, 1.);
pub const LIGHT_BLUE_200: Color = from_u32(0x81d4fa, 1.);
pub const LIGHT_BLUE_300: Color = from_u32(0x4fc3f7, 1.);
pub const LIGHT_BLUE_400: Color = from_u32(0x29b6f6, 1.);
pub const LIGHT_BLUE_500: Color = from_u32(0x03a9f4, 1.);
pub const LIGHT_BLUE_600: Color = from_u32(0x039be5, 1.);
pub const LIGHT_BLUE_700: Color = from_u32(0x0288d1, 1.);
pub const LIGHT_BLUE_800: Color = from_u32(0x0277bd, 1.);
pub const LIGHT_BLUE_900: Color = from_u32(0x01579b, 1.);
pub const LIGHT_BLUE_A100: Color = from_u32(0x80d8ff, 1.);
pub const LIGHT_BLUE_A200: Color = from_u32(0x40c4ff, 1.);
pub const LIGHT_BLUE_A400: Color = from_u32(0x00b0ff, 1.);
pub const LIGHT_BLUE_A700: Color = from_u32(0x0091ea, 1.);

pub const CYAN_50: Color = from_u32(0xe0f7fa, 1.);
pub const CYAN_100: Color = from_u32(0xb2ebf2, 1.);
pub const CYAN_200: Color = from_u32(0x80deea, 1.);
pub const CYAN_300: Color = from_u32(0x4dd0e1, 1.);
pub const CYAN_400: Color = from_u32(0x26c6da, 1.);
pub const CYAN_500: Color = from_u32(0x00bcd4, 1.);
pub const CYAN_600: Color = from_u32(0x00acc1, 1.);
pub const CYAN_700: Color = from_u32(0x0097a7, 1.);
pub const CYAN_800: Color = from_u32(0x00838f, 1.);
pub const CYAN_900: Color = from_u32(0x006064, 1.);
pub const CYAN_A100: Color = from_u32(0x84ffff, 1.);
pub const CYAN_A200: Color = from_u32(0x18ffff, 1.);
pub const CYAN_A400: Color = from_u32(0x00e5ff, 1.);
pub const CYAN_A700: Color = from_u32(0x00b8d4, 1.);

pub const TEAL_50: Color = from_u32(0xe0f2f1, 1.);
pub const TEAL_100: Color = from_u32(0xb2dfdb, 1.);
pub const TEAL_200: Color = from_u32(0x80cbc4, 1.);
pub const TEAL_300: Color = from_u32(0x4db6ac, 1.);
pub const TEAL_400: Color = from_u32(0x26a69a, 1.);
pub const TEAL_500: Color = from_u32(0x009688, 1.);
pub const TEAL_600: Color = from_u32(0x00897b, 1.);
pub const TEAL_700: Color = from_u32(0x00796b, 1.);
pub const TEAL_800: Color = from_u32(0x00695c, 1.);
pub const TEAL_900: Color = from_u32(0x004d40, 1.);
pub const TEAL_A100: Color = from_u32(0xa7ffeb, 1.);
pub const TEAL_A200: Color = from_u32(0x64ffda, 1.);
pub const TEAL_A400: Color = from_u32(0x1de9b6, 1.);
pub const TEAL_A700: Color = from_u32(0x00bfa5, 1.);

pub const GREEN_50: Color = from_u32(0xe8f5e9, 1.);
pub const GREEN_100: Color = from_u32(0xc8e6c9, 1.);
pub const GREEN_200: Color = from_u32(0xa5d6a7, 1.);
pub const GREEN_300: Color = from_u32(0x81c784, 1.);
pub const GREEN_400: Color = from_u32(0x66bb6a, 1.);
pub const GREEN_500: Color = from_u32(0x4caf50, 1.);
pub const GREEN_600: Color = from_u32(0x43a047, 1.);
pub const GREEN_700: Color = from_u32(0x388e3c, 1.);
pub const GREEN_800: Color = from_u32(0x2e7d32, 1.);
pub const GREEN_900: Color = from_u32(0x1b5e20, 1.);
pub const GREEN_A100: Color = from_u32(0xb9f6ca, 1.);
pub const GREEN_A200: Color = from_u32(0x69f0ae, 1.);
pub const GREEN_A400: Color = from_u32(0x00e676, 1.);
pub const GREEN_A700: Color = from_u32(0x00c853, 1.);

pub const LIGHT_GREEN_50: Color = from_u32(0xf1f8e9, 1.);
pub const LIGHT_GREEN_100: Color = from_u32(0xdcedc8, 1.);
pub const LIGHT_GREEN_200: Color = from_u32(0xc5e1a5, 1.);
pub const LIGHT_GREEN_300: Color = from_u32(0xaed581, 1.);
pub const LIGHT_GREEN_400: Color = from_u32(0x9ccc65, 1.);
pub const LIGHT_GREEN_500: Color = from_u32(0x8bc34a, 1.);
pub const LIGHT_GREEN_600: Color = from_u32(0x7cb342, 1.);
pub const LIGHT_GREEN_700: Color = from_u32(0x689f38, 1.);
pub const LIGHT_GREEN_800: Color = from_u32(0x558b2f, 1.);
pub const LIGHT_GREEN_900: Color = from_u32(0x33691e, 1.);
pub const LIGHT_GREEN_A100: Color = from_u32(0xccff90, 1.);
pub const LIGHT_GREEN_A200: Color = from_u32(0xb2ff59, 1.);
pub const LIGHT_GREEN_A400: Color = from_u32(0x76ff03, 1.);
pub const LIGHT_GREEN_A700: Color = from_u32(0x64dd17, 1.);

pub const LIME_50: Color = from_u32(0xf9fbe7, 1.);
pub const LIME_100: Color = from_u32(0xf0f4c3, 1.);
pub const LIME_200: Color = from_u32(0xe6ee9c, 1.);
pub const LIME_300: Color = from_u32(0xdce775, 1.);
pub const LIME_400: Color = from_u32(0xd4e157, 1.);
pub const LIME_500: Color = from_u32(0xcddc39, 1.);
pub const LIME_600: Color = from_u32(0xc0ca33, 1.);
pub const LIME_700: Color = from_u32(0xafb42b, 1.);
pub const LIME_800: Color = from_u32(0x9e9d24, 1.);
pub const LIME_900: Color = from_u32(0x827717, 1.);
pub const LIME_A100: Color = from_u32(0xf4ff81, 1.);
pub const LIME_A200: Color = from_u32(0xeeff41, 1.);
pub const LIME_A400: Color = from_u32(0xc6ff00, 1.);
pub const LIME_A700: Color = from_u32(0xaeea00, 1.);

pub const YELLOW_50: Color = from_u32(0xfffde7, 1.);
pub const YELLOW_100: Color = from_u32(0xfff9c4, 1.);
pub const YELLOW_200: Color = from_u32(0xfff59d, 1.);
pub const YELLOW_300: Color = from_u32(0xfff176, 1.);
pub const YELLOW_400: Color = from_u32(0xffee58, 1.);
pub const YELLOW_500: Color = from_u32(0xffeb3b, 1.);
pub const YELLOW_600: Color = from_u32(0xfdd835, 1.);
pub const YELLOW_700: Color = from_u32(0xfbc02d, 1.);
pub const YELLOW_800: Color = from_u32(0xf9a825, 1.);
pub const YELLOW_900: Color = from_u32(0xf57f17, 1.);
pub const YELLOW_A100: Color = from_u32(0xffff8d, 1.);
pub const YELLOW_A200: Color = from_u32(0xffff00, 1.);
pub const YELLOW_A400: Color = from_u32(0xffea00, 1.);
pub const YELLOW_A700: Color = from_u32(0xffd600, 1.);

pub const AMBER_50: Color = from_u32(0xfff8e1, 1.);
pub const AMBER_100: Color = from_u32(0xffecb3, 1.);
pub const AMBER_200: Color = from_u32(0xffe082, 1.);
pub const AMBER_300: Color = from_u32(0xffd54f, 1.);
pub const AMBER_400: Color = from_u32(0xffca28, 1.);
pub const AMBER_500: Color = from_u32(0xffc107, 1.);
pub const AMBER_600: Color = from_u32(0xffb300, 1.);
pub const AMBER_700: Color = from_u32(0xffa000, 1.);
pub const AMBER_800: Color = from_u32(0xff8f00, 1.);
pub const AMBER_900: Color = from_u32(0xff6f00, 1.);
pub const AMBER_A100: Color = from_u32(0xffe57f, 1.);
pub const AMBER_A200: Color = from_u32(0xffd740, 1.);
pub const AMBER_A400: Color = from_u32(0xffc400, 1.);
pub const AMBER_A700: Color = from_u32(0xffab00, 1.);

pub const ORANGE_50: Color = from_u32(0xfff3e0, 1.);
pub const ORANGE_100: Color = from_u32(0xffe0b2, 1.);
pub const ORANGE_200: Color = from_u32(0xffcc80, 1.);
pub const ORANGE_300: Color = from_u32(0xffb74d, 1.);
pub const ORANGE_400: Color = from_u32(0xffa726, 1.);
pub const ORANGE_500: Color = from_u32(0xff9800, 1.);
pub const ORANGE_600: Color = from_u32(0xfb8c00, 1.);
pub const ORANGE_700: Color = from_u32(0xf57c00, 1.);
pub const ORANGE_800: Color = from_u32(0xef6c00, 1.);
pub const ORANGE_900: Color = from_u32(0xe65100, 1.);
pub const ORANGE_A100: Color = from_u32(0xffd180, 1.);
pub const ORANGE_A200: Color = from_u32(0xffab40, 1.);
pub const ORANGE_A400: Color = from_u32(0xff9100, 1.);
pub const ORANGE_A700: Color = from_u32(0xff6d00, 1.);

pub const DEEP_ORANGE_50: Color = from_u32(0xfbe9e7, 1.);
pub const DEEP_ORANGE_100: Color = from_u32(0xffccbc, 1.);
pub const DEEP_ORANGE_200: Color = from_u32(0xffab91, 1.);
pub const DEEP_ORANGE_300: Color = from_u32(0xff8a65, 1.);
pub const DEEP_ORANGE_400: Color = from_u32(0xff7043, 1.);
pub const DEEP_ORANGE_500: Color = from_u32(0xff5722, 1.);
pub const DEEP_ORANGE_600: Color = from_u32(0xf4511e, 1.);
pub const DEEP_ORANGE_700: Color = from_u32(0xe64a19, 1.);
pub const DEEP_ORANGE_800: Color = from_u32(0xd84315, 1.);
pub const DEEP_ORANGE_900: Color = from_u32(0xbf360c, 1.);
pub const DEEP_ORANGE_A100: Color = from_u32(0xff9e80, 1.);
pub const DEEP_ORANGE_A200: Color = from_u32(0xff6e40, 1.);
pub const DEEP_ORANGE_A400: Color = from_u32(0xff3d00, 1.);
pub const DEEP_ORANGE_A700: Color = from_u32(0xdd2c00, 1.);

pub const BROWN_50: Color = from_u32(0xefebe9, 1.);
pub const BROWN_100: Color = from_u32(0xd7ccc8, 1.);
pub const BROWN_200: Color = from_u32(0xbcaaa4, 1.);
pub const BROWN_300: Color = from_u32(0xa1887f, 1.);
pub const BROWN_400: Color = from_u32(0x8d6e63, 1.);
pub const BROWN_500: Color = from_u32(0x795548, 1.);
pub const BROWN_600: Color = from_u32(0x6d4c41, 1.);
pub const BROWN_700: Color = from_u32(0x5d4037, 1.);
pub const BROWN_800: Color = from_u32(0x4e342e, 1.);
pub const BROWN_900: Color = from_u32(0x3e2723, 1.);

pub const GREY_50: Color = from_u32(0xfafafa, 1.);
pub const GREY_100: Color = from_u32(0xf5f5f5, 1.);
pub const GREY_200: Color = from_u32(0xeeeeee, 1.);
pub const GREY_300: Color = from_u32(0xe0e0e0, 1.);
pub const GREY_400: Color = from_u32(0xbdbdbd, 1.);
pub const GREY_500: Color = from_u32(0x9e9e9e, 1.);
pub const GREY_600: Color = from_u32(0x757575, 1.);
pub const GREY_700: Color = from_u32(0x616161, 1.);
pub const GREY_800: Color = from_u32(0x424242, 1.);
pub const GREY_900: Color = from_u32(0x212121, 1.);

pub const BLUE_GREY_50: Color = from_u32(0xeceff1, 1.);
pub const BLUE_GREY_100: Color = from_u32(0xcfd8dc, 1.);
pub const BLUE_GREY_200: Color = from_u32(0xb0bec5, 1.);
pub const BLUE_GREY_300: Color = from_u32(0x90a4ae, 1.);
pub const BLUE_GREY_400: Color = from_u32(0x78909c, 1.);
pub const BLUE_GREY_500: Color = from_u32(0x607d8b, 1.);
pub const BLUE_GREY_600: Color = from_u32(0x546e7a, 1.);
pub const BLUE_GREY_700: Color = from_u32(0x455a64, 1.);
pub const BLUE_GREY_800: Color = from_u32(0x37474f, 1.);
pub const BLUE_GREY_900: Color = from_u32(0x263238, 1.);
