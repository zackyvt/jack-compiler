pub static KEYWORDS: &'static [&'static str] = &[
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];
pub static SYMBOLS: &'static [char] = &[
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];

#[derive(Debug, Clone)]
// Represents a lexical token
pub enum Token {
    Keyword(&'static str),
    Symbol(char),
    IntConst(u16),
    StringConst(String),
    Identifier(String),
}

impl Token {
    // Return the XML representation string of the token
    pub fn as_xml(&self) -> String {
        match self {
            Token::Keyword(v) => format!("<keyword> {} </keyword>", v),
            Token::Symbol(v) => match v {
                '&' | '<' | '>' | '"' => format!(
                    "<symbol> {} </symbol>",
                    match v {
                        '&' => "&amp;",
                        '<' => "&lt;",
                        '>' => "&gt;",
                        '"' => "&quot;",
                        _ => unreachable!(),
                    }
                ),
                _ => format!("<symbol> {} </symbol>", v),
            },
            Token::IntConst(v) => format!("<integerConstant> {} </integerConstant>", v),
            Token::StringConst(v) => format!("<stringConstant> {} </stringConstant>", v),
            Token::Identifier(v) => format!("<identifier> {} </identifier>", v),
        }
    }
}
