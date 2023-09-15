use super::Syntax;
use std::collections::HashSet;

impl Syntax {
    #[must_use]
    pub fn lua() -> Syntax {
        Syntax {
            language: "Lua",
            case_sensitive: true,
            comment: "--",
            comment_multiline: ["--[[", "]]"],
            keywords: HashSet::from([
                "and", "break", "do", "else", "elseif", "end", "for", "function", "if",
                "in", "local","not", "or", "repeat", "return", "then", "until",
                "while",
            ]),
            types: HashSet::from([
                "boolean", "number", "string", "function", "userdata", "thread",
                "table",
            ]),
            special: HashSet::from([
                "false",
                "nil",
                "true",
            ]),
        }
    }
}