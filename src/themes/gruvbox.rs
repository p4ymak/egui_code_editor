use super::ColorTheme;

impl ColorTheme {
    /// Author : Jakub Bartodziej <kubabartodziej@gmail.com>
    /// The theme uses the gruvbox dark palette with standard contrast: github.com/morhetz/gruvbox
    pub const GRUVBOX: ColorTheme = ColorTheme {
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
    };

    pub const GRUVBOX_DARK: ColorTheme = ColorTheme::GRUVBOX;
}
