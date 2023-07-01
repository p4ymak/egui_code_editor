mod highlighting;
mod syntax;
mod themes;

use highlighting::highlight;
use std::hash::{Hash, Hasher};
pub use syntax::{Syntax, TokenType};
pub use themes::ColorTheme;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct CodeEditor {
    height: usize,
    theme: ColorTheme,
    fontsize: f32,
    numlines: bool,
    syntax: Syntax,
}

impl Hash for CodeEditor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.theme.hash(state);
        (self.fontsize as u32).hash(state);
        self.syntax.hash(state);
    }
}

impl Default for CodeEditor {
    fn default() -> CodeEditor {
        CodeEditor {
            height: 1,
            theme: ColorTheme::GRUVBOX,
            fontsize: 10.0,
            numlines: true,
            syntax: Syntax::sql(),
        }
    }
}
impl CodeEditor {
    #[allow(clippy::new_ret_no_self)]
    pub fn with_height(self, height: usize) -> CodeEditor {
        CodeEditor { height, ..self }
    }
    pub fn with_theme(self, theme: ColorTheme) -> CodeEditor {
        CodeEditor { theme, ..self }
    }
    pub fn with_fontsize(self, fontsize: f32) -> CodeEditor {
        CodeEditor { fontsize, ..self }
    }
    pub fn with_ui_fontsize(self, ui: &mut egui::Ui) -> CodeEditor {
        CodeEditor {
            fontsize: egui::TextStyle::Monospace.resolve(ui.style()).size,
            ..self
        }
    }
    pub fn with_numlines(self, numlines: bool) -> CodeEditor {
        CodeEditor { numlines, ..self }
    }
    pub fn with_syntax(self, syntax: Syntax) -> CodeEditor {
        CodeEditor { syntax, ..self }
    }

    pub fn format(&self, ty: TokenType) -> egui::text::TextFormat {
        let font_id = egui::FontId::monospace(self.fontsize);
        let color = self.theme.type_color(ty);
        egui::text::TextFormat::simple(font_id, color)
    }
    fn numlines_view(&self, ui: &mut egui::Ui, text: &str) {
        let total = if text.ends_with('\n') || text.is_empty() {
            text.lines().count() + 1
        } else {
            text.lines().count()
        }
        .max(self.height);
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

        let width = (max_indent * self.fontsize as usize) as f32 * 0.5;

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
        ui.style_mut().visuals.extreme_bg_color = self.theme.bg();
        ui.add(
            egui::TextEdit::multiline(&mut counter)
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_rows(self.height)
                .desired_width(width)
                .layouter(&mut layouter),
        );
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, text: &mut String) {
        egui::ScrollArea::vertical().show(ui, |v| {
            v.style_mut().override_font_id = Some(egui::FontId::monospace(self.fontsize));
            v.horizontal_top(|h| {
                if self.numlines {
                    self.numlines_view(h, text);
                }
                egui::ScrollArea::horizontal().show(h, |ui| {
                    let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                        let layout_job = highlight(ui.ctx(), self, string);
                        ui.fonts(|f| f.layout_job(layout_job))
                    };
                    ui.style_mut().visuals.extreme_bg_color = self.theme.bg();
                    ui.add(
                        egui::TextEdit::multiline(text)
                            .lock_focus(true)
                            .desired_rows(self.height)
                            .desired_width(f32::MAX)
                            .layouter(&mut layouter),
                    );
                });
            });
        });
    }
}

// macro_rules! font {
//     ($name: expr, $path: literal) => {
//         let mut fonts = egui::FontDefinitions::default();
//         fonts.font_data.insert(
//             $name.to_owned(),
//             egui::FontData::from_static(include_bytes!($path)),
//         );

//         fonts
//             .families
//             .insert(egui::FontFamily::Name($name.into()), vec![$name.to_owned()]);

//         fonts
//             .families
//             .get_mut(&egui::FontFamily::Proportional)
//             .unwrap()
//             .insert(0, $name.to_owned());

//         fonts
//             .families
//             .get_mut(&egui::FontFamily::Monospace)
//             .unwrap()
//             .insert(0, $name.to_owned());
//         fonts
//     };
// }
