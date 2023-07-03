#![allow(dead_code)]
pub mod ayu;
pub mod github;
pub mod gruvbox;
pub mod sonokai;

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
    pub cursor: &'static str,
    pub selection: &'static str,
    pub comments: &'static str,
    pub functions: &'static str,
    pub keywords: &'static str,
    pub literals: &'static str,
    pub numerics: &'static str,
    pub punctuation: &'static str,
    pub strs: &'static str,
    pub types: &'static str,
    pub special: &'static str,
}

impl ColorTheme {
    pub fn bg(&self) -> Color32 {
        color_from_hex(self.bg).unwrap_or(ERROR_COLOR)
    }
    pub fn cursor(&self) -> Color32 {
        color_from_hex(self.cursor).unwrap_or(ERROR_COLOR)
    }
    pub fn selection(&self) -> Color32 {
        color_from_hex(self.selection).unwrap_or(ERROR_COLOR)
    }

    pub fn style(&self) -> Style {
        let mut style = Style::default();
        style.visuals.widgets.noninteractive.bg_fill = self.bg();
        style.visuals.window_fill = self.bg();
        style.visuals.selection.stroke.color = self.cursor();
        style.visuals.selection.bg_fill = self.selection();
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
            TokenType::Special => self.special,
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
            TokenType::Special => color_from_hex(self.special),
            TokenType::Str => color_from_hex(self.strs),
            TokenType::Type => color_from_hex(self.types),
            TokenType::Whitespace => color_from_hex(self.bg),
        }
        .unwrap_or(ERROR_COLOR)
    }
    pub fn monocolor(
        dark: bool,
        bg: &'static str,
        fg: &'static str,
        cursor: &'static str,
        selection: &'static str,
    ) -> Self {
        ColorTheme {
            dark,
            bg,
            cursor,
            selection,
            literals: fg,
            numerics: fg,
            keywords: fg,
            functions: fg,
            punctuation: fg,
            types: fg,
            strs: fg,
            comments: fg,
            special: fg,
        }
    }
}
