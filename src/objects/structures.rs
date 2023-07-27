use super::*;
use crate::utils::*;

// Returns the class grouping
pub fn class(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("class");
    res.add_token(s.keyword("class")?);
    res.add_token(s.identifier()?);
    res.add_token(s.symbol('{')?);
    res.add_repeat_grouping(s, class_var_dec);
    res.add_repeat_grouping(s, subroutine_dec);
    res.add_token(s.symbol('}')?);
    Ok(res)
}

// Returns the type token
// Keyword: int/char/bool OR Identifier
fn type_dec(s: &mut TokenStream) -> TokenResult {
    let dtype = s.peek()?;
    match dtype {
        Token::Keyword("int") | Token::Keyword("char") | Token::Keyword("boolean") => {
            s.keywords(&["int", "char", "boolean"])
        }
        _ => s.identifier(),
    }
}

// Returns the class variable declaration grouping
fn class_var_dec(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("classVarDec");
    res.add_token(s.keywords(&["static", "field"])?);
    res.add_token(type_dec(s)?);
    res.add_comma_repeat_token(
        s,
        |s, r| {
            r.add_token(s.identifier()?);
            Ok(())
        },
        true,
    )?;
    res.add_token(s.symbol(';')?);
    Ok(res)
}

// Returns the subroutine declaration grouping
fn subroutine_dec(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("subroutineDec");
    res.add_token(s.keywords(&["constructor", "function", "method"])?);
    res.add_token({
        let void_t = s.keyword("void");
        match void_t {
            Ok(t) => t,
            Err(_) => type_dec(s)?,
        }
    });
    res.add_token(s.identifier()?);
    res.add_token(s.symbol('(')?);
    res.add_grouping(parameter_list(s)?);
    res.add_token(s.symbol(')')?);
    res.add_grouping(subroutine_body(s)?);
    Ok(res)
}

// Returns the subroutine body grouping
fn subroutine_body(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("subroutineBody");
    res.add_token(s.symbol('{')?);
    res.add_repeat_grouping(s, var_dec);
    res.add_grouping(statements(s)?);
    res.add_token(s.symbol('}')?);
    Ok(res)
}

// Returns the variable declaration grouping
fn var_dec(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("varDec");
    res.add_token(s.keyword("var")?);
    res.add_token(type_dec(s)?);
    res.add_comma_repeat_token(
        s,
        |s, r| {
            r.add_token(s.identifier()?);
            Ok(())
        },
        true,
    )?;
    res.add_token(s.symbol(';')?);
    Ok(res)
}

// Returns the parameter list token
fn parameter_list(s: &mut TokenStream) -> ParseResult {
    let mut res = Grouping::new("parameterList");
    res.add_comma_repeat_token(
        s,
        |s, r| {
            r.add_token(type_dec(s)?);
            r.add_token(s.identifier()?);
            Ok(())
        },
        false,
    )?;
    Ok(res)
}
