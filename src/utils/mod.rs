pub mod grouping;
pub mod symbol_table;
pub mod token;
pub mod token_stream;

pub use grouping::*;
pub use symbol_table::*;
pub use token::*;
pub use token_stream::*;

pub type TokenResult<'a> = Result<&'a Token, &'static str>;
pub type ParseResult = Result<Grouping, &'static str>;
