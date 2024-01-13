use super::*;

#[test]
fn numeric_float() {
    assert_eq!(
        Token::default().tokens(&Syntax::default(), "3.14"),
        [Token::new(TokenType::Numeric(true), "3.14")]
    );
}

#[test]
fn numeric_float_desription() {
    assert_eq!(
        Token::default().tokens(&Syntax::default(), "3.14_f32"),
        [
            Token::new(TokenType::Numeric(true), "3.14"),
            Token::new(TokenType::Numeric(true), "_"),
            Token::new(TokenType::Numeric(true), "f32")
        ]
    );
}

#[test]
fn numeric_float_second_dot() {
    assert_eq!(
        Token::default().tokens(&Syntax::default(), "3.14.032"),
        [
            Token::new(TokenType::Numeric(true), "3.14"),
            Token::new(TokenType::Punctuation('.'), "."),
            Token::new(TokenType::Numeric(false), "032")
        ]
    );
}

#[test]
fn simple_rust() {
    let syntax = Syntax::rust();
    let input = vec![
        Token::new(TokenType::Keyword, "fn"),
        Token::new(TokenType::Whitespace(' '), " "),
        Token::new(TokenType::Function, "function"),
        Token::new(TokenType::Punctuation('('), "("),
        Token::new(TokenType::Str('\"'), "\"String\""),
        Token::new(TokenType::Punctuation(')'), ")"),
        Token::new(TokenType::Punctuation('{'), "{"),
        Token::new(TokenType::Whitespace('\n'), "\n"),
        Token::new(TokenType::Whitespace('\t'), "\t"),
        Token::new(TokenType::Keyword, "let"),
        Token::new(TokenType::Whitespace(' '), " "),
        Token::new(TokenType::Literal, "x_0"),
        Token::new(TokenType::Punctuation(':'), ":"),
        Token::new(TokenType::Whitespace(' '), " "),
        Token::new(TokenType::Type, "f32"),
        Token::new(TokenType::Whitespace(' '), " "),
        Token::new(TokenType::Punctuation('='), "="),
        Token::new(TokenType::Numeric(true), "13.34"),
        Token::new(TokenType::Punctuation(';'), ";"),
        Token::new(TokenType::Whitespace('\n'), "\n"),
        Token::new(TokenType::Punctuation('}'), "}"),
    ];
    let str = input.iter().map(|h| h.buffer()).collect::<String>();
    let output = Token::default().tokens(&syntax, &str);
    println!("{str}");
    assert_eq!(input, output);
}
