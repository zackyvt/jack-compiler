use super::*;
use crate::utils::*;

pub fn expression(
    expression_grouping: &Grouping,
    symbol_table: &SymbolTable,
) -> Result<String, String> {
    if expression_grouping.subgroupings().len() == 1 {
        return term(expression_grouping.subgroupings()[0], symbol_table);
    }
    Ok(term(expression_grouping.subgroupings()[0], symbol_table)?
        + "\n"
        + &expression(&expression_grouping.slice(2), symbol_table)?
        + "\n"
        + match expression_grouping.tokens()[0] {
            Token::Symbol('+') => "add",
            Token::Symbol('-') => "sub",
            Token::Symbol('*') => "call Math.multiply",
            Token::Symbol('/') => "call Math.divide",
            Token::Symbol('&') => "and",
            Token::Symbol('|') => "or",
            Token::Symbol('<') => "lt",
            Token::Symbol('>') => "gt",
            Token::Symbol('=') => "eq",
            _ => {
                return Err("Missing operand between two terms".to_string());
            }
        })
}

fn expression_list(
    expression_list_grouping: &Grouping,
    symbol_table: &SymbolTable,
) -> Result<String, String> {
    Ok(expression_list_grouping
        .subgroupings()
        .iter()
        .map(|x| expression(x, symbol_table))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .fold(String::new(), |a, b| a + "\n" + &b))
}

fn term(term_grouping: &Grouping, symbol_table: &SymbolTable) -> Result<String, String> {
    let tokens = term_grouping.tokens();
    let groupings = term_grouping.subgroupings();
    Ok(match tokens[0] {
        Token::IntConst(v) => format!("push constant {}", v),
        Token::Keyword("null") | Token::Keyword("false") => format!("push constant 0"),
        Token::StringConst(s) => {
            format!("push constant {}\ncall String.new", s.len())
                + &s.chars()
                    .map(|x| format!("push constant {}\ncall String.appendChar", x as usize))
                    .fold(String::new(), |a, b| a + "\n" + &b)
        }
        Token::Keyword("true") => "push constant 1\nneg".to_string(),
        Token::Identifier(name) => "varname".to_string(),
        Token::Symbol(s) if (s == &'-' || s == &'~') => {
            term(groupings[0], symbol_table)?
                + "\n"
                + match s {
                    '-' => "neg",
                    '~' => "not",
                    _ => return Err("Invalid unary operator".to_string())?,
                }
        }
        Token::Symbol('(') => expression(groupings[0], symbol_table)?,
        _ => {
            return Err("Invalid expression term".to_string());
        }
    })
}
