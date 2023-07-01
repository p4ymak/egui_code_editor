#![allow(dead_code)]

use super::syntax::TokenType;
use egui::{Color32, Style};

pub const ERROR_COLOR: Color32 = Color32::from_rgb(255, 0, 255);

fn color_from_hex(hex: &str) -> Option<Color32> {
    if hex == "none" {
        return Some(Color32::from_rgba_premultiplied(255, 0, 255, 0));
    }
    let rgb = (1..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect::<Vec<u8>>();
    let color = Color32::from_rgb(*rgb.first()?, *rgb.get(1)?, *rgb.get(2)?);
    Some(color)
}

#[derive(Hash, Clone)]
pub struct ColorTheme {
    pub dark: bool,
    pub bg: &'static str,
    pub comments: &'static str,
    pub functions: &'static str,
    pub keywords: &'static str,
    pub literals: &'static str,
    pub numerics: &'static str,
    pub punctuation: &'static str,
    pub strs: &'static str,
    pub types: &'static str,
}

impl ColorTheme {
    pub fn bg(&self) -> Color32 {
        color_from_hex(self.bg).unwrap_or(ERROR_COLOR)
    }
    pub fn style(&self) -> Style {
        let mut style = Style::default();

        style.visuals.widgets.noninteractive.bg_fill = self.bg();
        style.visuals.window_fill = self.bg();
        style.visuals.selection.stroke.color = self.type_color(TokenType::Literal);
        style.visuals.extreme_bg_color = self.bg();
        style
    }
    pub const fn type_color_str(&self, ty: TokenType) -> &'static str {
        match ty {
            TokenType::Comment => self.comments,
            TokenType::Function => self.functions,
            TokenType::Keyword => self.keywords,
            TokenType::Literal => self.literals,
            TokenType::Numeric => self.numerics,
            TokenType::Punctuation => self.punctuation,
            TokenType::Str => self.strs,
            TokenType::Type => self.types,
            TokenType::Whitespace => self.bg,
        }
    }
    pub fn type_color(&self, ty: TokenType) -> Color32 {
        match ty {
            TokenType::Comment => color_from_hex(self.comments),
            TokenType::Function => color_from_hex(self.functions),
            TokenType::Keyword => color_from_hex(self.keywords),
            TokenType::Literal => color_from_hex(self.literals),
            TokenType::Numeric => color_from_hex(self.numerics),
            TokenType::Punctuation => color_from_hex(self.punctuation),
            TokenType::Str => color_from_hex(self.strs),
            TokenType::Type => color_from_hex(self.types),
            TokenType::Whitespace => color_from_hex(self.bg),
        }
        .unwrap_or(ERROR_COLOR)
    }
    pub fn monocolor(dark: bool, fg: &'static str, bg: &'static str) -> Self {
        ColorTheme {
            dark,
            bg,
            literals: fg,
            numerics: fg,
            keywords: fg,
            functions: fg,
            punctuation: fg,
            types: fg,
            strs: fg,
            comments: fg,
        }
    }

    pub const MONOKAI: ColorTheme = ColorTheme {
        dark: true,
        bg: "#2e2e2e",
        comments: "#d6d6d6",    // blue
        functions: "#b4d273",   // red
        keywords: "#fa595e",    // green
        literals: "#6c99bb",    // orange
        numerics: "#9e86c8",    // yellow
        punctuation: "#e87d3e", // pinky
        strs: "#b05279",        // white
        types: "#e5b567",       // pink
    };

    pub const SONOKAI: ColorTheme = ColorTheme {
        dark: true,
        bg: "#2c2e34",
        comments: "#e2e2e3",    // blue
        functions: "#9ed072",   // red
        keywords: "#fa595e",    // green
        literals: "#5cc4de",    // orange
        numerics: "#b39df3",    // yellow
        punctuation: "#f1874c", // pinky
        strs: "#ef9df3",        // white
        types: "#e7c664",       // pink
    };

    pub const GRUVBOX: ColorTheme = ColorTheme {
        dark: true,
        bg: "#282828",
        comments: "#928374",    // gray1
        functions: "#b8bb26",   // green1
        keywords: "#fb4934",    // red1
        literals: "#ebdbb2",    // fg1
        numerics: "#d3869b",    // purple1
        punctuation: "#fe8019", // orange1
        strs: "#8ec07c",        // aqua1
        types: "#fabd2f",       // yellow1
    };

    pub const GRUVBOX_DARK: ColorTheme = ColorTheme::GRUVBOX;

    pub const GRUVBOX_LIGHT: ColorTheme = ColorTheme {
        dark: false,
        bg: "#fbf1c7",
        comments: "#282828",    // blue
        functions: "#98971a",   // red
        keywords: "#9d0006",    // green
        literals: "#458588",    // orange
        numerics: "#b16286",    // yellow
        punctuation: "#d65d0e", // pinky
        strs: "#8f3f71",        // dark_gray
        types: "#d79921",       // transparent
    };

    pub const ONEDARK: ColorTheme = ColorTheme {
        dark: true,
        bg: "#282c34",
        comments: "#abb2bf",    // blue
        functions: "#98c379",   // red
        keywords: "#e06c75",    // green
        literals: "#61afef",    // orange
        numerics: "#b16286",    // yellow
        punctuation: "#d65d0e", // pinky
        strs: "#8f3f71",        // gray
        types: "#e5c07b",       // transparent
    };

    pub const SOLARIZED: ColorTheme = ColorTheme {
        dark: true,
        bg: "#fdf6e3",
        comments: "#586e75",    // blue
        functions: "#859900",   // red
        keywords: "#dc322f",    // green
        literals: "#268bd2",    // orange
        numerics: "#6c71c4",    // yellow
        punctuation: "#cb4b16", // pinky
        strs: "#d33682",        // gray
        types: "#b58900",       // transparent
    };

    pub const AYU: ColorTheme = ColorTheme {
        dark: false,
        bg: "#fafafa",
        comments: "#828c9a",    // gray
        functions: "#ffaa33",   // yellow
        keywords: "#fa8d3e",    // orange
        literals: "#5c6166",    // foreground
        numerics: "#a37acc",    // magenta
        punctuation: "#5c6166", // foreground
        strs: "#86b300",        // green
        types: "#399ee6",       // blue
    };
}
