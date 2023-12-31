use super::*;

#[test]
fn numeric_float() {
    assert_eq!(
        Highlighter::default().tokens(&Syntax::default(), "3.14"),
        [Highlighter::new(TokenType::Numeric(true), "3.14")]
    );
}

#[test]
fn numeric_float_desription() {
    assert_eq!(
        Highlighter::default().tokens(&Syntax::default(), "3.14_f32"),
        [
            Highlighter::new(TokenType::Numeric(true), "3.14"),
            Highlighter::new(TokenType::Numeric(true), "_"),
            Highlighter::new(TokenType::Numeric(true), "f32")
        ]
    );
}

#[test]
fn numeric_float_second_dot() {
    assert_eq!(
        Highlighter::default().tokens(&Syntax::default(), "3.14.032"),
        [
            Highlighter::new(TokenType::Numeric(true), "3.14"),
            Highlighter::new(TokenType::Punctuation('.'), "."),
            Highlighter::new(TokenType::Numeric(false), "032")
        ]
    );
}

#[test]
fn simple_rust() {
    let syntax = Syntax::rust();
    let input = vec![
        Highlighter::new(TokenType::Keyword, "fn"),
        Highlighter::new(TokenType::Whitespace(' '), " "),
        Highlighter::new(TokenType::Function, "function"),
        Highlighter::new(TokenType::Punctuation('('), "("),
        Highlighter::new(TokenType::Str('\"'), "\"String\""),
        Highlighter::new(TokenType::Punctuation(')'), ")"),
        Highlighter::new(TokenType::Punctuation('{'), "{"),
        Highlighter::new(TokenType::Whitespace('\n'), "\n"),
        Highlighter::new(TokenType::Whitespace('\t'), "\t"),
        Highlighter::new(TokenType::Keyword, "let"),
        Highlighter::new(TokenType::Whitespace(' '), " "),
        Highlighter::new(TokenType::Literal, "x_0"),
        Highlighter::new(TokenType::Punctuation(':'), ":"),
        Highlighter::new(TokenType::Whitespace(' '), " "),
        Highlighter::new(TokenType::Type, "f32"),
        Highlighter::new(TokenType::Whitespace(' '), " "),
        Highlighter::new(TokenType::Punctuation('='), "="),
        Highlighter::new(TokenType::Numeric(true), "13.34"),
        Highlighter::new(TokenType::Punctuation(';'), ";"),
        Highlighter::new(TokenType::Whitespace('\n'), "\n"),
        Highlighter::new(TokenType::Punctuation('}'), "}"),
    ];
    let str = input.iter().map(|h| h.buffer()).collect::<String>();
    let output = Highlighter::default().tokens(&syntax, &str);
    println!("{str}");
    assert_eq!(input, output);
}
