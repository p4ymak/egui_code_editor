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
use egui::text::LayoutJob;
#[cfg(feature = "egui")]
use egui::widgets::text_edit::TextEditOutput;
#[cfg(feature = "egui")]
use highlighting::highlight;
pub use highlighting::Token;
#[cfg(feature = "editor")]
use std::hash::{Hash, Hasher};
pub use syntax::{Syntax, TokenType};
pub use themes::ColorTheme;
pub use themes::DEFAULT_THEMES;

#[cfg(feature = "egui")]
pub trait Editor: Hash {
    fn append(&self, job: &mut LayoutJob, token: &Token);
    fn syntax(&self) -> &Syntax;
}

#[cfg(feature = "editor")]
#[derive(Clone, Debug, PartialEq)]
/// CodeEditor struct which stores settings for highlighting.
pub struct CodeEditor {
    id: String,
    theme: ColorTheme,
    syntax: Syntax,
    numlines: bool,
    numlines_shift: isize,
    numlines_only_natural: bool,
    fontsize: f32,
    rows: usize,
    vscroll: bool,
    stick_to_bottom: bool,
    desired_width: f32,
}

#[cfg(feature = "editor")]
impl Hash for CodeEditor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.theme.hash(state);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (self.fontsize as u32).hash(state);
        self.syntax.hash(state);
    }
}

#[cfg(feature = "editor")]
impl Default for CodeEditor {
    fn default() -> CodeEditor {
        CodeEditor {
            id: String::from("Code Editor"),
            theme: ColorTheme::GRUVBOX,
            syntax: Syntax::rust(),
            numlines: true,
            numlines_shift: 0,
            numlines_only_natural: false,
            fontsize: 10.0,
            rows: 10,
            vscroll: true,
            stick_to_bottom: false,
            desired_width: f32::INFINITY,
        }
    }
}

#[cfg(feature = "editor")]
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

    /// Shift lines numbering by this value
    ///
    /// **Default: 0**
    pub fn with_numlines_shift(self, numlines_shift: isize) -> Self {
        CodeEditor {
            numlines_shift,
            ..self
        }
    }

    /// Show lines numbering only above zero, useful for enabling numbering since nth row
    ///
    /// **Default: false**
    pub fn with_numlines_only_natural(self, numlines_only_natural: bool) -> Self {
        CodeEditor {
            numlines_only_natural,
            ..self
        }
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
        CodeEditor {
            desired_width: if shrink { 0.0 } else { self.desired_width },
            ..self
        }
    }

    /// Sets the desired width of the code editor
    ///
    /// **Default: `f32::INFINITY`**
    pub fn desired_width(self, width: f32) -> Self {
        CodeEditor {
            desired_width: width,
            ..self
        }
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
        use egui::TextBuffer;

        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(self.rows) as isize;
        let max_indent = total
            .to_string()
            .len()
            .max(!self.numlines_only_natural as usize * self.numlines_shift.to_string().len());
        let mut counter = (1..=total)
            .map(|i| {
                let num = i + self.numlines_shift;
                if num <= 0 && self.numlines_only_natural {
                    String::new()
                } else {
                    let label = num.to_string();
                    format!(
                        "{}{label}",
                        " ".repeat(max_indent.saturating_sub(label.len()))
                    )
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        #[allow(clippy::cast_precision_loss)]
        let width = max_indent as f32
            * self.fontsize
            * 0.5
            * !(total + self.numlines_shift <= 0 && self.numlines_only_natural) as u8 as f32;

        let mut layouter = |ui: &egui::Ui, text_buffer: &dyn TextBuffer, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                text_buffer.as_str().to_string(),
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
    pub fn show(&mut self, ui: &mut egui::Ui, text: &mut dyn egui::TextBuffer) -> TextEditOutput {
        use egui::TextBuffer;

        let mut text_edit_output: Option<TextEditOutput> = None;
        let mut code_editor = |ui: &mut egui::Ui| {
            ui.horizontal_top(|h| {
                self.theme.modify_style(h, self.fontsize);
                if self.numlines {
                    self.numlines_show(h, text.as_str());
                }
                egui::ScrollArea::horizontal()
                    .id_salt(format!("{}_inner_scroll", self.id))
                    .show(h, |ui| {
                        let mut layouter =
                            |ui: &egui::Ui, text_buffer: &dyn TextBuffer, _wrap_width: f32| {
                                let layout_job = highlight(ui.ctx(), self, text_buffer.as_str());
                                ui.fonts(|f| f.layout_job(layout_job))
                            };
                        let output = egui::TextEdit::multiline(text)
                            .id_source(&self.id)
                            .lock_focus(true)
                            .desired_rows(self.rows)
                            .frame(true)
                            .desired_width(self.desired_width)
                            .layouter(&mut layouter)
                            .show(ui);
                        text_edit_output = Some(output);
                    });
            });
        };
        if self.vscroll {
            egui::ScrollArea::vertical()
                .id_salt(format!("{}_outer_scroll", self.id))
                .stick_to_bottom(self.stick_to_bottom)
                .show(ui, code_editor);
        } else {
            code_editor(ui);
        }

        text_edit_output.expect("TextEditOutput should exist at this point")
    }
}

#[cfg(feature = "editor")]
#[cfg(feature = "egui")]
impl Editor for CodeEditor {
    fn append(&self, job: &mut LayoutJob, token: &Token) {
        job.append(token.buffer(), 0.0, self.format(token.ty()));
    }

    fn syntax(&self) -> &Syntax {
        &self.syntax
    }
}
