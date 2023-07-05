mod highlighting;
mod syntax;
mod themes;

use highlighting::highlight;
use std::hash::{Hash, Hasher};

pub use syntax::{Syntax, TokenType};
pub use themes::ColorTheme;

#[derive(Clone, Debug, PartialEq)]
pub struct CodeEditor {
    id: String,
    theme: ColorTheme,
    syntax: Syntax,
    numlines: bool,
    fontsize: f32,
    rows: usize,
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
        }
    }
}

impl CodeEditor {
    #[must_use]
    pub fn id_source(self, id_source: impl Into<String>) -> Self {
        CodeEditor {
            id: id_source.into(),
            ..self
        }
    }
    #[must_use]
    pub fn with_rows(self, rows: usize) -> Self {
        CodeEditor { rows, ..self }
    }
    #[must_use]
    pub fn with_theme(self, theme: ColorTheme) -> Self {
        CodeEditor { theme, ..self }
    }
    #[must_use]
    pub fn with_fontsize(self, fontsize: f32) -> Self {
        CodeEditor { fontsize, ..self }
    }
    #[must_use]
    pub fn with_ui_fontsize(self, ui: &mut egui::Ui) -> Self {
        CodeEditor {
            fontsize: egui::TextStyle::Monospace.resolve(ui.style()).size,
            ..self
        }
    }
    #[must_use]
    pub fn with_numlines(self, numlines: bool) -> Self {
        CodeEditor { numlines, ..self }
    }
    #[must_use]
    pub fn with_syntax(self, syntax: Syntax) -> Self {
        CodeEditor { syntax, ..self }
    }

    #[must_use]
    pub fn format(&self, ty: TokenType) -> egui::text::TextFormat {
        let font_id = egui::FontId::monospace(self.fontsize);
        let color = self.theme.type_color(ty);
        egui::text::TextFormat::simple(font_id, color)
    }
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
                    self.theme.type_color(TokenType::Comment),
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

    pub fn show(&mut self, ui: &mut egui::Ui, text: &mut String) -> egui::Response {
        let mut response: Option<egui::Response> = None;
        egui::ScrollArea::vertical()
            .id_source(format!("{}_outer_scroll", self.id))
            .show(ui, |ui| {
                self.theme.modify_style(ui, self.fontsize);
                ui.horizontal_top(|h| {
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
                            let resp = ui.add(
                                egui::TextEdit::multiline(text)
                                    .id_source(&self.id)
                                    .lock_focus(true)
                                    .desired_rows(self.rows)
                                    .frame(true)
                                    .desired_width(f32::MAX)
                                    .layouter(&mut layouter),
                            );
                            response = Some(resp);
                        });
                });
            });
        response.expect("response should exist at this point")
    }
}
