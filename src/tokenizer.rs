use crate::utils::{Token, KEYWORDS, SYMBOLS};

// Encode the tokens into an XML string
pub fn into_xml(tokens: Vec<Token>) -> String {
    format!(
        "<tokens>{}</tokens>",
        tokens
            .iter()
            .fold(String::new(), |acc, x| acc + &x.as_xml())
    )
}

// Removes comments and trims the leading and ending whitespace
pub fn compact_contents(contents: &str) -> String {
    let mut res = String::new();
    let mut commented = false;
    for line in contents.lines() {
        for (i, c) in line.chars().enumerate() {
            let next_c = line.chars().nth(i + 1).unwrap_or_default();
            let prev_c = {
                if i == 0 {
                    '0'
                } else {
                    line.chars().nth(i - 1).unwrap()
                }
            };
            if c == '/' && next_c == '/' {
                break;
            }
            if c == '/' && next_c == '*' {
                commented = true;
                continue;
            }
            if prev_c == '*' && c == '/' {
                commented = false;
                continue;
            }
            if !commented {
                res.push(c);
            }
        }
    }
    res.trim().to_string()
}

// Checks if arg is a valid lexical identifier token
fn is_valid_identifier(id: &str) -> bool {
    !id.is_empty()
        && !id.chars().nth(0).unwrap().is_numeric()
        && id.chars().all(|x| x.is_alphanumeric() || x == '_')
}

// Tokenizes an integer constant, keyword or identifier which is by itself (no spaces/other characters)
fn tokenize_word(word: &str) -> Result<Token, String> {
    if word.is_empty() {
        panic!("Empty string cannot be tokenized as a word");
    }
    if let Some(k) = KEYWORDS.iter().find(|&i| i == &word) {
        Ok(Token::Keyword(k))
    } else if let Ok(int) = word.parse::<u16>() {
        Ok(Token::IntConst(int))
    } else if is_valid_identifier(word) {
        Ok(Token::Identifier(word.to_string()))
    } else {
        Err(format!(
            "Unable to tokenize '{}' - not an integer constant, a keyword nor an identifier",
            word
        ))
    }
}

// Tokenizes the compacted source code (source code with no comments)
fn tokenize_compacted(input: &str) -> Result<Vec<Token>, String> {
    let mut res = vec![];
    let mut temp = String::new();
    let mut is_literal = false;
    for c in input.trim().chars() {
        if is_literal {
            if c == '"' {
                is_literal = false;
                res.push(Token::StringConst(temp));
                temp = String::new();
            } else {
                temp.push(c);
            }
            continue;
        }

        let symbol = SYMBOLS.iter().find(|&i| i == &c);
        if c.is_whitespace() || symbol.is_some() || c == '"' {
            if !temp.is_empty() {
                res.push(tokenize_word(&temp)?);
                temp.clear();
            }
            if let Some(&s) = symbol {
                res.push(Token::Symbol(s));
            }
            if c == '"' {
                is_literal = true;
            }
            continue;
        }
        temp.push(c);
    }
    Ok(res)
}

// Tokenizes the raw input source code
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    tokenize_compacted(&compact_contents(input))
}

// Tokenizes the raw input source code into an XML file
pub fn tokenize_into_xml(input: &str) -> Result<String, String> {
    Ok(into_xml(tokenize(input)?))
}
