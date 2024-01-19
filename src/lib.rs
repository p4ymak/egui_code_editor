#![allow(rustdoc::invalid_rust_codeblocks)]
//! Text Editor Widget for [egui](https://github.com/emilk/egui) with numbered lines and simple syntax highlighting based on keywords sets.
//!
//! ## Usage with egui
//!
//! ```rust
//! use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
//!
//! CodeEditor::default()
//!   .id_source("code editor")
//!   .with_rows(12)
//!   .with_fontsize(14.0)
//!   .with_theme(ColorTheme::GRUVBOX)
//!   .with_syntax(Syntax::rust())
//!   .with_numlines(true)
//!   .show(ui, &mut self.code);
//! ```
//!
//! ## Usage as lexer without egui
//!
//! **Cargo.toml**
//!
//! ```toml
//! [dependencies]
//! egui_code_editor = { version = "0.2" , default-features = false }
//! colorful = "0.2.2"
//! ```
//!
//! **main.rs**
//!
//! ```rust
//! use colorful::{Color, Colorful};
//! use egui_code_editor::{Syntax, Token, TokenType};
//!
//! fn color(token: TokenType) -> Color {
//!     match token {
//!         TokenType::Comment(_) => Color::Grey37,
//!         TokenType::Function => Color::Yellow3b,
//!         TokenType::Keyword => Color::IndianRed1c,
//!         TokenType::Literal => Color::NavajoWhite1,
//!         TokenType::Numeric(_) => Color::MediumPurple,
//!         TokenType::Punctuation(_) => Color::Orange3,
//!         TokenType::Special => Color::Cyan,
//!         TokenType::Str(_) => Color::Green,
//!         TokenType::Type => Color::GreenYellow,
//!         TokenType::Whitespace(_) => Color::White,
//!         TokenType::Unknown => Color::Pink1,
//!     }
//! }
//!
//! fn main() {
//!     let text = r#"// Code Editor
//! CodeEditor::default()
//!     .id_source("code editor")
//!     .with_rows(12)
//!     .with_fontsize(14.0)
//!     .with_theme(self.theme)
//!     .with_syntax(self.syntax.to_owned())
//!     .with_numlines(true)
//!     .vscroll(true)
//!     .show(ui, &mut self.code);
//!     "#;
//!
//!     let syntax = Syntax::rust();
//!     for token in Token::default().tokens(&syntax, text) {
//!         print!("{}", token.buffer().color(color(token.ty())));
//!     }
//! }
//! ```

pub mod highlighting;
mod syntax;
#[cfg(test)]
mod tests;
mod themes;

#[cfg(feature = "egui")]
use egui::widgets::text_edit::TextEditOutput;
#[cfg(feature = "egui")]
use highlighting::highlight;
pub use highlighting::Token;
use std::hash::{Hash, Hasher};
pub use syntax::{Syntax, TokenType};
pub use themes::ColorTheme;
pub use themes::DEFAULT_THEMES;

#[derive(Clone, Debug, PartialEq)]
/// CodeEditor struct which stores settings for highlighting.
pub struct CodeEditor {
    id: String,
    theme: ColorTheme,
    syntax: Syntax,
    numlines: bool,
    fontsize: f32,
    rows: usize,
    vscroll: bool,
    stick_to_bottom: bool,
    shrink: bool,
}

impl Hash for CodeEditor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.theme.hash(state);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (self.fontsize as u32).hash(state);
        self.syntax.hash(state);
    }
}

impl Default for CodeEditor {
    fn default() -> CodeEditor {
        CodeEditor {
            id: String::from("Code Editor"),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
            numlines: true,
            fontsize: 10.0,
            rows: 10,
            vscroll: true,
            stick_to_bottom: false,
            shrink: false,
        }
    }
}

impl CodeEditor {
    pub fn id_source(self, id_source: impl Into<String>) -> Self {
        CodeEditor {
            id: id_source.into(),
            ..self
        }
    }

    /// Minimum number of rows to show.
    ///
    /// **Default: 10**
    pub fn with_rows(self, rows: usize) -> Self {
        CodeEditor { rows, ..self }
    }

    /// Use custom Color Theme
    ///
    /// **Default: Gruvbox**
    pub fn with_theme(self, theme: ColorTheme) -> Self {
        CodeEditor { theme, ..self }
    }

    /// Use custom font size
    ///
    /// **Default: 10.0**
    pub fn with_fontsize(self, fontsize: f32) -> Self {
        CodeEditor { fontsize, ..self }
    }

    #[cfg(feature = "egui")]
    /// Use UI font size
    pub fn with_ui_fontsize(self, ui: &mut egui::Ui) -> Self {
        CodeEditor {
            fontsize: egui::TextStyle::Monospace.resolve(ui.style()).size,
            ..self
        }
    }

    /// Show or hide lines numbering
    ///
    /// **Default: true**
    pub fn with_numlines(self, numlines: bool) -> Self {
        CodeEditor { numlines, ..self }
    }

    /// Use custom syntax for highlighting
    ///
    /// **Default: Rust**
    pub fn with_syntax(self, syntax: Syntax) -> Self {
        CodeEditor { syntax, ..self }
    }

    /// Turn on/off scrolling on the vertical axis.
    ///
    /// **Default: true**
    pub fn vscroll(self, vscroll: bool) -> Self {
        CodeEditor { vscroll, ..self }
    }
    /// Should the containing area shrink if the content is small?
    ///
    /// **Default: false**
    pub fn auto_shrink(self, shrink: bool) -> Self {
        CodeEditor { shrink, ..self }
    }

    /// Stick to bottom
    /// The scroll handle will stick to the bottom position even while the content size
    /// changes dynamically. This can be useful to simulate terminal UIs or log/info scrollers.
    /// The scroll handle remains stuck until user manually changes position. Once "unstuck"
    /// it will remain focused on whatever content viewport the user left it on. If the scroll
    /// handle is dragged to the bottom it will again become stuck and remain there until manually
    /// pulled from the end position.
    ///
    /// **Default: false**
    pub fn stick_to_bottom(self, stick_to_bottom: bool) -> Self {
        CodeEditor {
            stick_to_bottom,
            ..self
        }
    }

    #[cfg(feature = "egui")]
    pub fn format(&self, ty: TokenType) -> egui::text::TextFormat {
        let font_id = egui::FontId::monospace(self.fontsize);
        let color = self.theme.type_color(ty);
        egui::text::TextFormat::simple(font_id, color)
    }

    #[cfg(feature = "egui")]
    fn numlines_show(&self, ui: &mut egui::Ui, text: &str) {
        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(self.rows);
        let max_indent = total.to_string().len();
        let mut counter = (1..=total)
            .map(|i| {
                let label = i.to_string();
                format!(
                    "{}{label}",
                    " ".repeat(max_indent.saturating_sub(label.len()))
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        #[allow(clippy::cast_precision_loss)]
        let width = max_indent as f32 * self.fontsize * 0.5;

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                string.to_string(),
                egui::TextFormat::simple(
                    egui::FontId::monospace(self.fontsize),
                    self.theme.type_color(TokenType::Comment(true)),
                ),
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut counter)
                .id_source(format!("{}_numlines", self.id))
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_rows(self.rows)
                .desired_width(width)
                .layouter(&mut layouter),
        );
    }

    #[cfg(feature = "egui")]
    /// Show Code Editor
    pub fn show(&mut self, ui: &mut egui::Ui, text: &mut String) -> TextEditOutput {
        let mut text_edit_output: Option<TextEditOutput> = None;
        let mut code_editor = |ui: &mut egui::Ui| {
            ui.horizontal_top(|h| {
                self.theme.modify_style(h, self.fontsize);
                if self.numlines {
                    self.numlines_show(h, text);
                }
                egui::ScrollArea::horizontal()
                    .id_source(format!("{}_inner_scroll", self.id))
                    .show(h, |ui| {
                        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                            let layout_job = highlight(ui.ctx(), self, string);
                            ui.fonts(|f| f.layout_job(layout_job))
                        };
                        let output = egui::TextEdit::multiline(text)
                            .id_source(&self.id)
                            .lock_focus(true)
                            .desired_rows(self.rows)
                            .frame(true)
                            .desired_width(if self.shrink { 0.0 } else { f32::MAX })
                            .layouter(&mut layouter)
                            .show(ui);
                        text_edit_output = Some(output);
                    });
            });
        };
        if self.vscroll {
            egui::ScrollArea::vertical()
                .id_source(format!("{}_outer_scroll", self.id))
                .stick_to_bottom(self.stick_to_bottom)
                .show(ui, code_editor);
        } else {
            code_editor(ui);
        }

        text_edit_output.expect("TextEditOutput should exist at this point")
    }
}
