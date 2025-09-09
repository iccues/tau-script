use frontend_library::token::{comment::Comment, identifier::Identifier, keyword::Keyword, number::{Float, Integer}, operator::Operator, singleton_token::EofToken, string::StringToken};
use lexer::get_lexer;

const INPUT: &str = r#"
// This is a short comment
/*
This is a long comment
*/

a = 10;
b = 20;
sum = a + b;

c = 30.5;
d = "Hello, World!";
e = true;

if (a == b) {
    g = 11;
} else {
    g = 12;
};
"#;

#[test]
fn test_lexer() {
    let mut lexer = get_lexer(INPUT.as_bytes());

    // Comments
    assert!(lexer.next().unwrap().eq(&*Comment::new(Some(" This is a short comment".to_string()))));
    assert!(lexer.next().unwrap().eq(&*Comment::new(Some("\nThis is a long comment\n".to_string()))));

    // a = 10;
    assert!(lexer.next().unwrap().eq(&*Identifier::new("a".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Integer::new("10".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // b = 20;
    assert!(lexer.next().unwrap().eq(&*Identifier::new("b".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Integer::new("20".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // sum = x + y;
    assert!(lexer.next().unwrap().eq(&*Identifier::new("sum".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("a".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Plus));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("b".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // c = 30.5;
    assert!(lexer.next().unwrap().eq(&*Identifier::new("c".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Float::new("30.5".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // d = "Hello, World!";
    assert!(lexer.next().unwrap().eq(&*Identifier::new("d".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*StringToken::new("\"Hello, World!\"".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // e = true;
    assert!(lexer.next().unwrap().eq(&*Identifier::new("e".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&Keyword::True));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // if (a == b) {
    //     g = 11;
    // }
    assert!(lexer.next().unwrap().eq(&Keyword::If));
    assert!(lexer.next().unwrap().eq(&Operator::OpenParen));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("a".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::DoubleEq));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("b".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::CloseParen));
    assert!(lexer.next().unwrap().eq(&Operator::OpenBrace));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("g".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Integer::new("11".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));
    assert!(lexer.next().unwrap().eq(&Operator::CloseBrace));

    // else {
    //     g = 12;
    // };
    assert!(lexer.next().unwrap().eq(&Keyword::Else));
    assert!(lexer.next().unwrap().eq(&Operator::OpenBrace));
    assert!(lexer.next().unwrap().eq(&*Identifier::new("g".to_string())));
    assert!(lexer.next().unwrap().eq(&Operator::Eq));
    assert!(lexer.next().unwrap().eq(&*Integer::new("12".to_string()).unwrap()));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));
    assert!(lexer.next().unwrap().eq(&Operator::CloseBrace));
    assert!(lexer.next().unwrap().eq(&Operator::Semi));

    // EOF
    assert!(lexer.next().unwrap().eq(&EofToken));
}
