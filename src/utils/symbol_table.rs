use super::*;
use std::fmt;

#[derive(Clone)]
pub enum SymbolKind {
    StaticVar,
    FieldVar,
    ArgumentVar,
    LocalVar,
}

#[derive(Clone)]
pub enum Datatype {
    Boolean,
    Char,
    Int,
    Custom(String),
}

impl Datatype {
    pub fn keyword_to_datatype(keyword: &str) -> Datatype {
        match keyword {
            "bool" => Datatype::Boolean,
            "char" => Datatype::Char,
            "int" => Datatype::Int,
            _ => Datatype::Custom(keyword.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Symbol<'a> {
    name: &'a str,
    dtype: Datatype,
    kind: SymbolKind,
    index: usize,
}

impl<'a> Symbol<'a> {
    pub fn class(&self) -> String {
        match &self.dtype {
            Datatype::Boolean => "bool".to_string(),
            Datatype::Char => "char".to_string(),
            Datatype::Int => "int".to_string(),
            Datatype::Custom(s) => s.to_string(),
        }
    }

    pub fn literal(&self) -> String {
        (match self.kind {
            SymbolKind::StaticVar => "static",
            SymbolKind::FieldVar => "field",
            SymbolKind::ArgumentVar => "argument",
            SymbolKind::LocalVar => "local",
        })
        .to_string()
            + " "
            + &format!("{}", self.index)
    }
}

pub struct SymbolTable<'a> {
    parent: Option<&'a SymbolTable<'a>>,
    symbols: Vec<Symbol<'a>>,
}

impl<'a> SymbolTable<'a> {
    fn new(mut symbols: Vec<Symbol<'a>>) -> SymbolTable<'a> {
        let mut static_count = 0;
        let mut local_count = 0;
        let mut field_count = 0;
        let mut arg_count = 0;
        for symbol in &mut symbols {
            match symbol.kind {
                SymbolKind::StaticVar => {
                    symbol.index = static_count;
                    static_count += 1;
                }
                SymbolKind::LocalVar => {
                    symbol.index = local_count;
                    local_count += 1;
                }
                SymbolKind::FieldVar => {
                    symbol.index = field_count;
                    field_count += 1;
                }
                SymbolKind::ArgumentVar => {
                    symbol.index = arg_count;
                    arg_count += 1;
                }
            }
        }
        SymbolTable { parent: None, symbols }
    }

    pub fn get(&self, symbol_name: &str) -> Result<&Symbol<'_>, String> {
        for symbol in &self.symbols {
            if symbol.name == symbol_name {
                return Ok(symbol);
            }
        }
        if let Some(st) = self.parent {
            return st.get(symbol_name); 
        } else {
            return Err(format!("Undefined symbol {}", symbol_name));
        }
    }
}

impl fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Symbol Table");
        for symbol in &self.symbols {
            let dtype_str = match &symbol.dtype {
                Datatype::Boolean => "bool",
                Datatype::Char => "char",
                Datatype::Int => "int",
                Datatype::Custom(s) => &s,
            };
            let kind_str = match symbol.kind {
                SymbolKind::StaticVar => "static",
                SymbolKind::FieldVar => "field",
                SymbolKind::ArgumentVar => "arg",
                SymbolKind::LocalVar => "local",
            };
            writeln!(
                f,
                "{} {} {} {}",
                kind_str, symbol.index, dtype_str, symbol.name
            )?;
        }
        Ok(())
    }
}

fn token_to_datatype(token: &Token) -> Result<Datatype, String> {
    match token {
        Token::Keyword(x) => Ok(Datatype::keyword_to_datatype(x)),
        Token::Identifier(x) => Ok(Datatype::keyword_to_datatype(x)),
        _ => Err("Class variable declaration has no datatype".to_string()),
    }
}

fn class_var_symbols(class_var_grouping: &Grouping) -> Result<Vec<Symbol>, String> {
    if class_var_grouping.name != "classVarDec" {
        return Err("Grouping is not a class variable declaration".to_string());
    }
    let kind = match class_var_grouping.tokens()[0] {
        Token::Keyword("field") => SymbolKind::FieldVar,
        Token::Keyword("static") => SymbolKind::StaticVar,
        _ => {
            return Err("Class variable must be either static or field".to_string());
        }
    };
    let dtype = token_to_datatype(&class_var_grouping.tokens()[1])?;
    let tokens = class_var_grouping.tokens();
    tokens[2..tokens.len()]
        .into_iter()
        .filter(|x| matches!(x, Token::Identifier(_)))
        .map(|x| {
            Ok(Symbol {
                name: {
                    if let Token::Identifier(n) = x {
                        &n
                    } else {
                        Err("Invalid class variable declaration".to_string())?
                    }
                },
                dtype: dtype.clone(),
                kind: kind.clone(),
                index: 0,
            })
        })
        .collect()
}

pub fn create_symbol_table(class_grouping: &Grouping) -> Result<SymbolTable, String> {
    if class_grouping.name != "class" {
        return Err("Grouping is not a class".to_string());
    }
    let class_name = {
        if let Token::Identifier(name) = class_grouping.tokens()[1] {
            name
        } else {
            return Err("Class doesn't have a name".to_string())?;
        }
    };
    let symbols = class_grouping
        .subgroupings()
        .iter()
        .filter(|x| x.name == "classVarDec")
        .map(|x| class_var_symbols(x))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok(SymbolTable::new(symbols))
}

fn parameter_symbols(parameter_list: &Grouping) -> Result<Vec<Symbol>, String> {
    if parameter_list.name != "parameterList" {
        return Err("Grouping is not a parameter list".to_string());
    }
    parameter_list
        .tokens()
        .iter()
        .filter(|x| !matches!(x, Token::Symbol(',')))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .map(|(type_token, name_token)| {
            Ok(Symbol {
                name: {
                    if let Token::Identifier(name) = name_token {
                        name
                    } else {
                        Err("Parameter list is invalid".to_string())?
                    }
                },
                dtype: token_to_datatype(&type_token)?,
                kind: SymbolKind::ArgumentVar,
                index: 0,
            })
        })
        .collect::<Result<Vec<_>, String>>()
}

fn var_dec_symbols(var_dec: &Grouping) -> Result<Vec<Symbol>, String> {
    if var_dec.name != "varDec" {
        return Err("Grouping is not a local variable declaration".to_string());
    }
    let dtype = token_to_datatype(&var_dec.tokens()[1])?;
    var_dec
        .tokens()
        .iter()
        .filter(|x| matches!(x, Token::Identifier(_)))
        .map(|x| {
            Ok(Symbol {
                name: {
                    if let Token::Identifier(n) = x {
                        &n
                    } else {
                        Err("Invalid local variable declaration".to_string())?
                    }
                },
                dtype: dtype.clone(),
                kind: SymbolKind::LocalVar,
                index: 0,
            })
        })
        .collect()
}

fn create_subroutine_symbol_table(
    subroutine_grouping: &Grouping,
    class_name: String,
) -> Result<SymbolTable, String> {
    if subroutine_grouping.name != "subroutineDec" {
        return Err("Not a subroutine grouping".to_string());
    }
    let mut args = parameter_symbols(subroutine_grouping.subgroupings()[0])?;
    if let Token::Keyword("method") = subroutine_grouping.tokens()[0] {
        let this_symbol = Symbol {
            name: "this",
            dtype: Datatype::Custom(class_name),
            kind: SymbolKind::ArgumentVar,
            index: 0,
        };
        args.insert(0, this_symbol);
    }
    let locals = subroutine_grouping.subgroupings()[1]
        .subgroupings()
        .iter()
        .filter(|x| x.name == "varDec")
        .map(|x| var_dec_symbols(x))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(SymbolTable::new([args, locals].concat()))
}
