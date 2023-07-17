use super::ColorTheme;

impl ColorTheme {
    /// Author : Jakub Bartodziej <kubabartodziej@gmail.com>
    /// Theme uses the gruvbox dark palette with standard contrast <https://github.com/morhetz/gruvbox>
    pub const GRUVBOX: ColorTheme = ColorTheme {
        name: "Gruvbox",
        dark: true,
        bg: "#282828",
        cursor: "#a89984",      // fg4
        selection: "#504945",   // bg2
        comments: "#928374",    // gray1
        functions: "#b8bb26",   // green1
        keywords: "#fb4934",    // red1
        literals: "#ebdbb2",    // fg1
        numerics: "#d3869b",    // purple1
        punctuation: "#fe8019", // orange1
        strs: "#8ec07c",        // aqua1
        types: "#fabd2f",       // yellow1
        special: "#83a598",     // blue1
    };

    pub const GRUVBOX_DARK: ColorTheme = ColorTheme::GRUVBOX;

    pub const GRUVBOX_LIGHT: ColorTheme = ColorTheme {
        name: "Gruvbox Light",
        dark: false,
        bg: "#fbf1c7",
        cursor: "#7c6f64",      // fg4
        selection: "#b57614",   // yellow1
        comments: "#7c6f64",    // gray1
        functions: "#79740e",   // green1
        keywords: "#9d0006",    // red1
        literals: "#282828",    // fg1
        numerics: "#8f3f71",    // purple1
        punctuation: "#af3a03", // orange1
        strs: "#427b58",        // aqua1
        types: "#b57614",       // yellow1
        special: "#af3a03",     // orange1
    };
}
