use super::ColorTheme;

impl ColorTheme {
    /// Author : OwOSwordsman <owoswordsman@gmail.com>
    /// An unofficial GitHub theme, generated using colors from: <https://primer.style/primitives/colors>
    pub const GITHUB_DARK: ColorTheme = ColorTheme {
        name: "Github Dark",
        dark: true,
        bg: "#0d1117",          // default
        cursor: "#d29922",      // attention.fg
        selection: "#0c2d6b",   // scale.blue.8
        comments: "#8b949e",    // fg.muted
        functions: "#d2a8ff",   // scale.purple.2
        keywords: "#ff7b72",    // scale.red.3
        literals: "#c9d1d9",    // fg.default
        numerics: "#79c0ff",    // scale.blue.2
        punctuation: "#c9d1d9", // fg.default
        strs: "#a5d6ff",        // scale.blue.1
        types: "#ffa657",       // scale.orange.2
        special: "#a5d6ff",     // scale.blue.1
    };

    pub const GITHUB_LIGHT: ColorTheme = ColorTheme {
        name: "Github Light",
        dark: false,
        bg: "#ffffff",          // default
        cursor: "#000000",      // invert
        selection: "#0550ae",   // scale.blue.6
        comments: "#57606a",    // fg.muted
        functions: "#8250df",   // done.fg
        keywords: "#cf222e",    // scale.red.5
        literals: "#24292f",    // fg.default
        numerics: "#0550ae",    // scale.blue.6
        punctuation: "#24292f", // fg.default
        strs: "#0a3069",        // scale.blue.8
        types: "#953800",       // scale.orange.6
        special: "#a475f9",     // scale.purple.4
    };
}
