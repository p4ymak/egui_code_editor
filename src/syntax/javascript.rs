use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn javascript() -> Syntax {
        Syntax {
            language: "JavaScript",
            case_sensitive: true,
            comment: "//",
            comment_multiline: ["/*", "*/"],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                // Control flow
                "if",
                "else",
                "switch",
                "case",
                "default",
                "break",
                "continue",
                "return",
                "throw",
                "try",
                "catch",
                "finally",
                // Loops
                "for",
                "while",
                "do",
                // Declarations
                "var",
                "let",
                "const",
                "function",
                "class",
                "extends",
                "import",
                "export",
                "from",
                "as",
                "default",
                // Operators
                "new",
                "delete",
                "typeof",
                "instanceof",
                "in",
                "of",
                "void",
                // Async
                "async",
                "await",
                "yield",
                // Other
                "this",
                "super",
                "static",
                "get",
                "set",
                "with",
                "debugger",
            ]),
            types: BTreeSet::from([
                // Built-in objects
                "Object",
                "Function",
                "Boolean",
                "Symbol",
                "Error",
                "AggregateError",
                "EvalError",
                "RangeError",
                "ReferenceError",
                "SyntaxError",
                "TypeError",
                "URIError",
                // Numbers
                "Number",
                "BigInt",
                "Math",
                "Date",
                // Text
                "String",
                "RegExp",
                // Collections
                "Array",
                "Map",
                "Set",
                "WeakMap",
                "WeakSet",
                // Typed Arrays
                "Int8Array",
                "Uint8Array",
                "Uint8ClampedArray",
                "Int16Array",
                "Uint16Array",
                "Int32Array",
                "Uint32Array",
                "Float32Array",
                "Float64Array",
                "BigInt64Array",
                "BigUint64Array",
                "ArrayBuffer",
                "SharedArrayBuffer",
                "DataView",
                // Control abstraction
                "Promise",
                "Generator",
                "GeneratorFunction",
                "AsyncFunction",
                "AsyncGenerator",
                "AsyncGeneratorFunction",
                // Reflection
                "Reflect",
                "Proxy",
                // Internationalization
                "Intl",
                // WebAssembly
                "WebAssembly",
                // Structured data
                "JSON",
                "Atomics",
            ]),
            special: BTreeSet::from([
                "true",
                "false",
                "null",
                "undefined",
                "NaN",
                "Infinity",
                "globalThis",
                "arguments",
            ]),
        }
    }
}
