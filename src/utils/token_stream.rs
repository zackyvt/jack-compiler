use super::{Token, TokenResult};
use std::fmt;

// Represents a stream of tokens
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens[self.pos])
    }
}

impl TokenStream {
    // Create a new TokenStream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // Return the next token without advancing the stream
    pub fn peek(&self) -> TokenResult {
        return self.tokens.get(self.pos).ok_or("No more tokens left");
    }

    // Validate next token in stream based on validation function and return it
    pub fn next(&mut self, validation_f: impl Fn(&Token) -> bool) -> TokenResult {
        let token = self.tokens.get(self.pos).ok_or("No more tokens left")?;
        if validation_f(token) {
            // println!("suc {:?}", token);
            self.pos += 1;
            Ok(token)
        } else {
            // println!("fail {:?}", token);
            Err("Token does not match format")
        }
    }

    // Return next token assuming its a specific keyword
    pub fn keyword(&mut self, keyword: &'static str) -> TokenResult {
        // println!("keyword val {}", keyword);
        self.next(|t| matches!(t, Token::Keyword(k) if k == &keyword))
    }

    // Return next token assuming its a specific keyword
    pub fn keywords(&mut self, keywords: &'static [&'static str]) -> TokenResult {
        // println!("keywords val {:?}", keywords);
        self.next(|t| {
            keywords
                .iter()
                .any(|keyword| matches!(t, Token::Keyword(k) if k == keyword))
        })
    }

    // Return next token assuming its a specific symbol
    pub fn symbol(&mut self, symbol: char) -> TokenResult {
        // println!("symbol val {}", symbol);
        self.next(|t| matches!(t, Token::Symbol(s) if s == &symbol))
    }

    // Return next token assuming its a string constant
    pub fn string(&mut self) -> TokenResult {
        // println!("string const val");
        self.next(|t| matches!(t, Token::StringConst(_)))
    }

    // Return next token assuming its an integer
    pub fn int(&mut self) -> TokenResult {
        // println!("int const val");
        self.next(|t| matches!(t, Token::IntConst(_)))
    }

    // Return next token assuming its an identifier
    pub fn identifier(&mut self) -> TokenResult {
        self.next(|t| matches!(t, Token::Identifier(_)))
    }
}
