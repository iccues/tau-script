use frontend_library::token::singleton_token::EofToken;
use lexer::get_lexer;
use parser::parse_stmt;

const INPUT: &str = r#"
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
fn test_parser() {
    let mut lexer = get_lexer(INPUT.as_bytes());

    // a = 10;
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        r#"Expr(Binary(BinaryExpr { left: Identifier("a"), operator: Ref(Eq), right: Literal(Integer(10)) }))"#
    );

    // b = 20;
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        r#"Expr(Binary(BinaryExpr { left: Identifier("b"), operator: Ref(Eq), right: Literal(Integer(20)) }))"#
    );

    // sum = a + b;
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        concat!(
            r#"Expr(Binary(BinaryExpr { left: Identifier("sum"), operator: Ref(Eq), "#,
            r#"right: Binary(BinaryExpr { left: Identifier("a"), operator: Ref(Plus), right: Identifier("b") }) }))"#
        )
    );

    // c = 30.5;
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        r#"Expr(Binary(BinaryExpr { left: Identifier("c"), operator: Ref(Eq), right: Literal(Float(30.5)) }))"#
    );

    // d = "Hello, World!";
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        r#"Expr(Binary(BinaryExpr { left: Identifier("d"), operator: Ref(Eq), right: Literal(String("Hello, World!")) }))"#
    );

    // e = true;
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        r#"Expr(Binary(BinaryExpr { left: Identifier("e"), operator: Ref(Eq), right: Literal(Bool(true)) }))"#
    );

    // if (a == b) {
    //     g = 11;
    // } else {
    //     g = 12;
    // };
    assert_eq!(
        format!("{:?}", parse_stmt(&mut lexer).unwrap()),
        concat!(
            r#"Expr(If(IfExpr { condition: Binary(BinaryExpr { left: Identifier("a"), operator: Ref(DoubleEq), right: Identifier("b") }), "#,
            r#"then_block: Block(Block { statments: [Binary(BinaryExpr { left: Identifier("g"), operator: Ref(Eq), right: Literal(Integer(11)) })], end_value: None }), "#,
            r#"else_block: Some(Block(Block { statments: [Binary(BinaryExpr { left: Identifier("g"), operator: Ref(Eq), right: Literal(Integer(12)) })], end_value: None })) }))"#
        )
    );

    // EOF
    assert!(lexer.next().unwrap().eq(&EofToken));
}
