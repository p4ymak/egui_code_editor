use super::Syntax;
use std::collections::HashSet;

impl Syntax {
    #[must_use]
    pub fn rust() -> Self {
        Syntax {
            language: "Rust",
            case_sensitive: true,
            comment: "//",
            keywords: HashSet::from([
                "as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for",
                "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
                "return", "self", "struct", "super", "trait", "type", "use", "where", "while",
                "async", "await", "abstract", "become", "box", "do", "final", "macro", "override",
                "priv", "typeof", "unsized", "virtual", "yield", "try", "unsafe", "dyn",
            ]),
            types: HashSet::from([
                "bool", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128	", "u128",
                "isize", "usize", "f32", "f64", "str", "String", "Vec",
            ]),
            special: HashSet::from(["Self", "static", "true", "false"]),
        }
    }
}
