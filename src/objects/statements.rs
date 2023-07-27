use super::*;
use crate::utils::*;

// Returns a statements grouping
pub fn statements(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("statements");
    res.add_repeat_grouping(s, statement);
    Ok(res)
}

// Returns one of the statement groupings
pub fn statement(s: &mut TokenStream) -> ParseResult {
    let statement_id = s.peek()?;
    match statement_id {
        Token::Keyword("let") => let_statement(s),
        Token::Keyword("if") => if_statement(s),
        Token::Keyword("while") => while_statement(s),
        Token::Keyword("do") => do_statement(s),
        Token::Keyword("return") => return_statement(s),
        _ => Err("Invalid statement identifier"),
    }
}

// Returns a let statement grouping
fn let_statement(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("letStatement");
    res.add_token(s.keyword("let")?);
    res.add_token(s.identifier()?);
    let square_bracket = s.symbol('[');
    if let Ok(b) = square_bracket {
        res.add_token(b);
        res.add_grouping(expression(s)?);
        res.add_token(s.symbol(']')?);
    }
    res.add_token(s.symbol('=')?);
    res.add_grouping(expression(s)?);
    res.add_token(s.symbol(';')?);
    Ok(res)
}

// Returns an if statement grouping
fn if_statement(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("ifStatement");
    res.add_token(s.keyword("if")?);
    res.add_token(s.symbol('(')?);
    res.add_grouping(expression(s)?);
    res.add_token(s.symbol(')')?);
    res.add_token(s.symbol('{')?);
    res.add_grouping(statements(s)?);
    res.add_token(s.symbol('}')?);
    let else_clause = s.keyword("else");
    if let Ok(c) = else_clause {
        res.add_token(c);
        res.add_token(s.symbol('{')?);
        res.add_grouping(statements(s)?);
        res.add_token(s.symbol('}')?);
    }
    Ok(res)
}

// Returns a while statement grouping
fn while_statement(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("whileStatement");
    res.add_token(s.keyword("while")?);
    res.add_token(s.symbol('(')?);
    res.add_grouping(expression(s)?);
    res.add_token(s.symbol(')')?);
    res.add_token(s.symbol('{')?);
    res.add_grouping(statements(s)?);
    res.add_token(s.symbol('}')?);
    Ok(res)
}

// Return a do statement grouping
fn do_statement(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("doStatement");
    res.add_token(s.keyword("do")?);
    res.add_token(s.identifier()?);
    if let Token::Symbol('.') = s.peek()? {
        res.add_token(s.symbol('.')?);
        res.add_token(s.identifier()?);
    }
    res.add_token(s.symbol('(')?);
    res.add_grouping(expression_list(s)?);
    res.add_token(s.symbol(')')?);
    res.add_token(s.symbol(';')?);
    Ok(res)
}

// Return a return statement grouping
fn return_statement(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("returnStatement");
    res.add_token(s.keyword("return")?);
    let exp = expression(s);
    if let Ok(e) = exp {
        res.add_grouping(e);
    }
    res.add_token(s.symbol(';')?);
    Ok(res)
}
