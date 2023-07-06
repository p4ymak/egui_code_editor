use super::ColorTheme;

impl ColorTheme {
    /// Author: André Sá <enkodr@outlook.com>
    ///
    /// Based on the AYU theme colors from <https://github.com/dempfi/ayu>
    pub const AYU: ColorTheme = ColorTheme {
        name: "Ayu",
        dark: false,
        bg: "#fafafa",
        cursor: "#5c6166",      // foreground
        selection: "#fa8d3e",   // orange
        comments: "#828c9a",    // gray
        functions: "#ffaa33",   // yellow
        keywords: "#fa8d3e",    // orange
        literals: "#5c6166",    // foreground
        numerics: "#a37acc",    // magenta
        punctuation: "#5c6166", // foreground
        strs: "#86b300",        // green
        types: "#399ee6",       // blue
        special: "#f07171",     // red
    };

    pub const AYU_MIRAGE: ColorTheme = ColorTheme {
        name: "Ayu Mirage",
        dark: true,
        bg: "#1f2430",
        cursor: "#cccac2",      // foreground
        selection: "#ffad66",   // orange
        comments: "#565b66",    // gray
        functions: "#ffcc77",   // yellow
        keywords: "#ffad66",    // orange
        literals: "#cccac2",    // foreground
        numerics: "#dfbfff",    // magenta
        punctuation: "#cccac2", // foreground
        strs: "#d5ff80",        // green
        types: "#73d0ff",       // blue
        special: "#f28779",     // red
    };

    pub const AYU_DARK: ColorTheme = ColorTheme {
        name: "Ayu Dark",
        dark: true,
        bg: "#0f1419",
        cursor: "#bfbdb6",      // foreground
        selection: "#ffad66",   // orange
        comments: "#5c6773",    // gray
        functions: "#e6b450",   // yellow
        keywords: "#ffad66",    // orange
        literals: "#bfbdb6",    // foreground
        numerics: "#dfbfff",    // magenta
        punctuation: "#bfbdb6", // foreground
        strs: "#aad94c",        // green
        types: "#59c2ff",       // blue
        special: "#f28779",     // red
    };
}
