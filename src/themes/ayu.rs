use super::ColorTheme;

impl ColorTheme {
    /// Author: André Sá <enkodr@outlook.com>
    /// Based on the AYU theme colors from https://github.com/dempfi/ayu
    pub const AYU: ColorTheme = ColorTheme {
        dark: false,
        bg: "#fafafa",
        cursor: "#fa8d3e",      // orange
        selection: "#d8d8d7",   // darg_gray
        comments: "#828c9a",    // gray
        functions: "#ffaa33",   // yellow
        keywords: "#fa8d3e",    // orange
        literals: "#5c6166",    // foreground
        numerics: "#a37acc",    // magenta
        punctuation: "#5c6166", // foreground
        strs: "#86b300",        // green
        types: "#399ee6",       // blue
        special: "#fa8d3e",     // orange
    };
}
