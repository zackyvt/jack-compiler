use crate::objects::structures;
use crate::tokenizer::tokenize;
use crate::utils::{Grouping, TokenStream};

// Parses the contents of a .jack file into an abstract syntax tree Grouping
pub fn parse(contents: &str) -> Result<Grouping, String> {
    let tokens = tokenize(contents)?;
    let mut stream = TokenStream::new(tokens);
    Ok(structures::class(&mut stream)?)
}

// Parses the conents of a .jack file into an XML abstract syntax tree
pub fn parse_into_xml(contents: &str) -> Result<String, String> {
    Ok(parse(contents)?.as_xml())
}
