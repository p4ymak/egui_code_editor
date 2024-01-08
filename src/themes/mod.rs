#![allow(dead_code)]
pub mod ayu;
pub mod github;
pub mod gruvbox;
pub mod sonokai;

use super::syntax::TokenType;
use egui::Color32;

pub const ERROR_COLOR: Color32 = Color32::from_rgb(255, 0, 255);

/// Array of default themes.
pub const DEFAULT_THEMES: [ColorTheme; 8] = [
    ColorTheme::AYU,
    ColorTheme::AYU_MIRAGE,
    ColorTheme::AYU_DARK,
    ColorTheme::GITHUB_DARK,
    ColorTheme::GITHUB_LIGHT,
    ColorTheme::GRUVBOX,
    ColorTheme::GRUVBOX_LIGHT,
    ColorTheme::SONOKAI,
];

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Colors in hexadecimal notation as used in HTML and CSS.
pub struct ColorTheme {
    pub name: &'static str,
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
impl Default for ColorTheme {
    fn default() -> Self {
        ColorTheme::GRUVBOX
    }
}
impl ColorTheme {
    #[must_use]
    pub fn name(&self) -> &str {
        self.name
    }
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.dark
    }
    #[must_use]
    pub fn bg(&self) -> Color32 {
        Color32::from_hex(self.bg).unwrap_or(ERROR_COLOR)
    }
    #[must_use]
    pub fn cursor(&self) -> Color32 {
        Color32::from_hex(self.cursor).unwrap_or(ERROR_COLOR)
    }
    #[must_use]
    pub fn selection(&self) -> Color32 {
        Color32::from_hex(self.selection).unwrap_or(ERROR_COLOR)
    }

    pub fn modify_style(&self, ui: &mut egui::Ui, fontsize: f32) {
        let style = ui.style_mut();
        style.visuals.widgets.noninteractive.bg_fill = self.bg();
        style.visuals.window_fill = self.bg();
        style.visuals.selection.stroke.color = self.cursor();
        style.visuals.selection.bg_fill = self.selection();
        style.visuals.extreme_bg_color = self.bg();
        style.override_font_id = Some(egui::FontId::monospace(fontsize));
        style.visuals.text_cursor.width = fontsize * 0.1;
    }

    #[must_use]
    pub const fn type_color_str(&self, ty: TokenType) -> &'static str {
        match ty {
            TokenType::Comment(_) => self.comments,
            TokenType::Function => self.functions,
            TokenType::Keyword => self.keywords,
            TokenType::Literal => self.literals,
            TokenType::Numeric(_) => self.numerics,
            TokenType::Punctuation(_) => self.punctuation,
            TokenType::Special => self.special,
            TokenType::Str(_) => self.strs,
            TokenType::Type => self.types,
            TokenType::Whitespace(_) | TokenType::Unknown => self.comments,
        }
    }

    #[must_use]
    pub fn type_color(&self, ty: TokenType) -> Color32 {
        match ty {
            TokenType::Comment(_) => Color32::from_hex(self.comments),
            TokenType::Function => Color32::from_hex(self.functions),
            TokenType::Keyword => Color32::from_hex(self.keywords),
            TokenType::Literal => Color32::from_hex(self.literals),
            TokenType::Numeric(_) => Color32::from_hex(self.numerics),
            TokenType::Punctuation(_) => Color32::from_hex(self.punctuation),
            TokenType::Special => Color32::from_hex(self.special),
            TokenType::Str(_) => Color32::from_hex(self.strs),
            TokenType::Type => Color32::from_hex(self.types),
            TokenType::Whitespace(_) | TokenType::Unknown => Color32::from_hex(self.comments),
        }
        .unwrap_or(ERROR_COLOR)
    }

    #[must_use]
    pub fn monocolor(
        dark: bool,
        bg: &'static str,
        fg: &'static str,
        cursor: &'static str,
        selection: &'static str,
    ) -> Self {
        ColorTheme {
            name: "monocolor",
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
