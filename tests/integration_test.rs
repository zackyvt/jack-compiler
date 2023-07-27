extern crate lib;

use lib::{parse_into_xml, tokenize_into_xml};

static src_files: [&'static str; 7] = [
    include_str!("./samples/ArrayTest/Main.jack"),
    include_str!("./samples/ExpressionLessSquare/Main.jack"),
    include_str!("./samples/ExpressionLessSquare/Square.jack"),
    include_str!("./samples/ExpressionLessSquare/SquareGame.jack"),
    include_str!("./samples/Square/Main.jack"),
    include_str!("./samples/Square/Square.jack"),
    include_str!("./samples/Square/SquareGame.jack"),
];

static token_files: [&'static str; 7] = [
    include_str!("./samples/ArrayTest/MainT.xml"),
    include_str!("./samples/ExpressionLessSquare/MainT.xml"),
    include_str!("./samples/ExpressionLessSquare/SquareT.xml"),
    include_str!("./samples/ExpressionLessSquare/SquareGameT.xml"),
    include_str!("./samples/Square/MainT.xml"),
    include_str!("./samples/Square/SquareT.xml"),
    include_str!("./samples/Square/SquareGameT.xml"),
];

static parse_files: [&'static str; 7] = [
    include_str!("./samples/ArrayTest/Main.xml"),
    include_str!("./samples/ExpressionLessSquare/Main.xml"),
    include_str!("./samples/ExpressionLessSquare/Square.xml"),
    include_str!("./samples/ExpressionLessSquare/SquareGame.xml"),
    include_str!("./samples/Square/Main.xml"),
    include_str!("./samples/Square/Square.xml"),
    include_str!("./samples/Square/SquareGame.xml"),
];

// Return true if the two strings are equal ignoring whitespace, case and newlines
fn text_eq(str_1: &str, str_2: &str) -> bool {
    let f = |s: &str| {
        s.replace('\n', "")
            .replace('\r', "")
            .replace(' ', "")
            .to_lowercase()
    };
    f(str_1) == f(str_2)
}

#[test]
// Test the tokenizing into XML process
fn tokenizer_test() {
    for (s, t) in src_files.iter().zip(token_files.iter()) {
        assert!(text_eq(&tokenize_into_xml(s).unwrap(), t));
    }
}

#[test]
// Test the parsing into XML process
fn parser_test() {
    for (s, p) in src_files.iter().zip(parse_files.iter()) {
        assert!(text_eq(&parse_into_xml(s).unwrap(), p));
    }
}
