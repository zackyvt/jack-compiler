use crate::utils::*;

// Returns an expression grouping
pub fn expression(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("expression");
    res.add_grouping(term(s)?);
    loop {
        let op = s.next(|op| match op {
            Token::Symbol('+')
            | Token::Symbol('-')
            | Token::Symbol('*')
            | Token::Symbol('/')
            | Token::Symbol('&')
            | Token::Symbol('|')
            | Token::Symbol('<')
            | Token::Symbol('>')
            | Token::Symbol('=') => true,
            _ => false,
        });
        match op {
            Ok(t) => {
                res.add_token(t);
                res.add_grouping(term(s)?);
            }
            _ => break,
        }
    }
    Ok(res)
}

// Returns a term grouping
pub fn term(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("term");
    let t = s.peek()?;
    match t {
        Token::IntConst(_)
        | Token::StringConst(_)
        | Token::Keyword("true")
        | Token::Keyword("false")
        | Token::Keyword("null")
        | Token::Keyword("this") => {
            res.add_token(s.next(|_| true)?);
        }
        Token::Identifier(_) => {
            res.add_token(s.identifier()?);
            let next = s.peek()?;
            match next {
                Token::Symbol('[') => {
                    res.add_token(s.symbol('[')?);
                    res.add_grouping(expression(s)?);
                    res.add_token(s.symbol(']')?);
                }
                Token::Symbol('(') => {
                    res.add_token(s.symbol('(')?);
                    res.add_grouping(expression_list(s)?);
                    res.add_token(s.symbol(')')?);
                }
                Token::Symbol('.') => {
                    res.add_token(s.symbol('.')?);
                    res.add_token(s.identifier()?);
                    res.add_token(s.symbol('(')?);
                    res.add_grouping(expression_list(s)?);
                    res.add_token(s.symbol(')')?);
                }
                _ => (),
            };
        }
        Token::Symbol('(') => {
            res.add_token(s.symbol('(')?);
            res.add_grouping(expression(s)?);
            res.add_token(s.symbol(')')?);
        }
        Token::Symbol('-') | Token::Symbol('~') => {
            res.add_token(s.next(|_| true)?);
            res.add_grouping(term(s)?);
        }
        _ => Err("Invalid term token")?,
    }
    Ok(res)
}

// Returns an expression list grouping
pub fn expression_list(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("expressionList");
    res.add_comma_repeat_token(
        s,
        |s, res| {
            res.add_grouping(expression(s)?);
            Ok(())
        },
        false,
    )?;
    Ok(res)
}
