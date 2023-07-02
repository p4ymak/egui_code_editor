use super::ColorTheme;

impl ColorTheme {
    pub const GITHUB_LIGHT: ColorTheme = ColorTheme {
        dark: false,
        bg: "#ffffff",          // default
        cursor: "#000000",      // invert
        selection: "#ddf4ff",   // scale.blue.0
        comments: "#57606a",    // fg.muted
        functions: "#8250df",   // done.fg
        keywords: "#cf222e",    // scale.red.5
        literals: "#24292f",    // fg.default
        numerics: "#0550ae",    // scale.blue.6
        punctuation: "#24292f", // fg.default
        strs: "#0a3069",        // scale.blue.8
        types: "#953800",       // scale.orange.6
    };
}
