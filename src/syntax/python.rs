use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn python() -> Syntax {
        Syntax {
            language: "Python",
            case_sensitive: true,
            comment: "#",
            comment_multiline: [r#"'''"#, r#"'''"#],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "and", "as", "assert", "break", "class", "continue", "def", "del", "elif", "else",
                "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda",
                "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with",
                "yield",
            ]),
            types: BTreeSet::from([
                "bool",
                "int",
                "float",
                "complex",
                "str",
                "list",
                "tuple",
                "range",
                "bytes",
                "bytearray",
                "memoryview",
                "dict",
                "set",
                "frozenset",
            ]),
            special: BTreeSet::from(["False", "None", "True"]),
        }
    }
}
