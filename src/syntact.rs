// #[cfg(feature = "syntect")]
// #[derive(Clone, Copy, Hash, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[allow(unused)]
// enum SyntectTheme {
//     Base16EightiesDark,
//     Base16MochaDark,
//     Base16OceanDark,
//     Base16OceanLight,
//     InspiredGitHub,
//     SolarizedDark,
//     SolarizedLight,
// }

// #[cfg(feature = "syntect")]
// #[allow(dead_code)]
// impl SyntectTheme {
//     fn all() -> impl ExactSizeIterator<Item = Self> {
//         [
//             Self::Base16EightiesDark,
//             Self::Base16MochaDark,
//             Self::Base16OceanDark,
//             Self::Base16OceanLight,
//             Self::InspiredGitHub,
//             Self::SolarizedDark,
//             Self::SolarizedLight,
//         ]
//         .iter()
//         .copied()
//     }

//     fn name(&self) -> &'static str {
//         match self {
//             Self::Base16EightiesDark => "Base16 Eighties (dark)",
//             Self::Base16MochaDark => "Base16 Mocha (dark)",
//             Self::Base16OceanDark => "Base16 Ocean (dark)",
//             Self::Base16OceanLight => "Base16 Ocean (light)",
//             Self::InspiredGitHub => "InspiredGitHub (light)",
//             Self::SolarizedDark => "Solarized (dark)",
//             Self::SolarizedLight => "Solarized (light)",
//         }
//     }

//     fn syntect_key_name(&self) -> &'static str {
//         match self {
//             Self::Base16EightiesDark => "base16-eighties.dark",
//             Self::Base16MochaDark => "base16-mocha.dark",
//             Self::Base16OceanDark => "base16-ocean.dark",
//             Self::Base16OceanLight => "base16-ocean.light",
//             Self::InspiredGitHub => "InspiredGitHub",
//             Self::SolarizedDark => "Solarized (dark)",
//             Self::SolarizedLight => "Solarized (light)",
//         }
//     }

//     pub fn is_dark(&self) -> bool {
//         match self {
//             Self::Base16EightiesDark
//             | Self::Base16MochaDark
//             | Self::Base16OceanDark
//             | Self::SolarizedDark => true,

//             Self::Base16OceanLight | Self::InspiredGitHub | Self::SolarizedLight => false,
//         }
//     }
// }

// #[cfg(feature = "syntect")]
// #[allow(dead_code)]
// impl CodeTheme {
//     pub fn dark(fontsize: u32) -> Self {
//         Self {
//             fontsize,
//             dark_mode: true,
//             syntect_theme: SyntectTheme::Base16MochaDark,
//         }
//     }

//     pub fn light(fontsize: u32) -> Self {
//         Self {
//             fontsize,
//             dark_mode: false,
//             syntect_theme: SyntectTheme::SolarizedLight,
//         }
//     }

//     pub fn ui(&mut self, ui: &mut egui::Ui) {
//         egui::widgets::global_dark_light_mode_buttons(ui);

//         for theme in SyntectTheme::all() {
//             if theme.is_dark() == self.dark_mode {
//                 ui.radio_value(&mut self.syntect_theme, theme, theme.name());
//             }
//         }
//     }
// }

// #[cfg(not(feature = "syntect"))]

// ----------------------------------------------------------------------------

// #[cfg(feature = "syntect")]
// struct Highlighter {
//     ps: syntect::parsing::SyntaxSet,
//     ts: syntect::highlighting::ThemeSet,
// }

// #[cfg(feature = "syntect")]
// impl Default for Highlighter {
//     fn default() -> Self {
//         Self {
//             ps: syntect::parsing::SyntaxSet::load_defaults_newlines(),
//             ts: syntect::highlighting::ThemeSet::load_defaults(),
//         }
//     }
// }

// #[cfg(feature = "syntect")]
// impl Highlighter {
//     #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
//     fn highlight(&self, theme: &CodeTheme, code: &str, lang: &str, fontsize: u32) -> LayoutJob {
//         self.highlight_impl(theme, code, lang, fontsize)
//             .unwrap_or_else(|| {
//                 // Fallback:
//                 LayoutJob::simple(
//                     code.into(),
//                     egui::FontId::monospace(fontsize as f32),
//                     if theme.dark_mode {
//                         egui::Color32::LIGHT_GRAY
//                     } else {
//                         egui::Color32::DARK_GRAY
//                     },
//                     f32::INFINITY,
//                 )
//             })
//     }

//     fn highlight_impl(
//         &self,
//         theme: &CodeTheme,
//         text: &str,
//         language: &str,
//         fontsize: u32,
//     ) -> Option<LayoutJob> {
//         use syntect::easy::HighlightLines;
//         use syntect::highlighting::FontStyle;
//         use syntect::util::LinesWithEndings;

//         let syntax = self
//             .ps
//             .find_syntax_by_name(language)
//             .or_else(|| self.ps.find_syntax_by_extension(language))?;

//         let theme = theme.syntect_theme.syntect_key_name();
//         let mut h = HighlightLines::new(syntax, &self.ts.themes[theme]);

//         use egui::text::{LayoutSection, TextFormat};

//         let mut job = LayoutJob {
//             text: text.into(),
//             ..Default::default()
//         };

//         for line in LinesWithEndings::from(text) {
//             for (style, range) in h.highlight_line(line, &self.ps).ok()? {
//                 let fg = style.foreground;
//                 let text_color = egui::Color32::from_rgb(fg.r, fg.g, fg.b);
//                 let italics = style.font_style.contains(FontStyle::ITALIC);
//                 let underline = style.font_style.contains(FontStyle::ITALIC);
//                 let underline = if underline {
//                     egui::Stroke::new(1.0, text_color)
//                 } else {
//                     egui::Stroke::NONE
//                 };
//                 job.sections.push(LayoutSection {
//                     leading_space: 0.0,
//                     byte_range: as_byte_range(text, range),
//                     format: TextFormat {
//                         font_id: egui::FontId::monospace(fontsize as f32),
//                         color: text_color,
//                         italics,
//                         underline,
//                         ..Default::default()
//                     },
//                 });
//             }
//         }

//         Some(job)
//     }
// }

// #[cfg(feature = "syntect")]
// fn as_byte_range(whole: &str, range: &str) -> std::ops::Range<usize> {
//     let whole_start = whole.as_ptr() as usize;
//     let range_start = range.as_ptr() as usize;
//     assert!(whole_start <= range_start);
//     assert!(range_start + range.len() <= whole_start + whole.len());
//     let offset = range_start - whole_start;
//     offset..(offset + range.len())
// }

// ----------------------------------------------------------------------------

// #[cfg(not(feature = "syntect"))]
