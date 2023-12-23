use super::ColorTheme;

impl ColorTheme {
    ///  Original Author: sainnhe <https://github.com/sainnhe/sonokai>
    ///  Modified by p4ymak <https://github.com/p4ymak>
    pub const SONOKAI: ColorTheme = ColorTheme {
        name: "Sonokai",
        dark: true,
        bg: "#2c2e34",          // bg0
        cursor: "#76cce0",      // blue
        selection: "#444852",   // bg5
        comments: "#7f8490",    // gray
        functions: "#9ed072",   // green
        keywords: "#fc5d7c",    // red
        literals: "#e2e2e3",    // foreground
        numerics: "#b39df3",    // purple
        punctuation: "#7f8490", // gray
        strs: "#e7c664",        // yellow
        types: "#399ee6",       // blue
        special: "#f39660",     // orange
    };
}
