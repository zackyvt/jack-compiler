use super::{ParseResult, Token, TokenStream};
use std::fmt;

#[derive(Clone)]
// Represents a grouping of tokens as a program element
pub struct Grouping {
    pub name: &'static str,
    pub items: Vec<GroupItem>,
}

impl fmt::Debug for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Grouping {
    // Create a new Grouping
    pub fn new(name: &'static str) -> Self {
        Grouping {
            name,
            items: vec![],
        }
    }

    // Create new grouping with first n items removed
    pub fn slice(&self, n: usize) -> Grouping {
        Grouping {
            name: self.name,
            items: self.items[n..].to_vec(),
        }
    }

    // Add a token to the grouping
    pub fn add_token(&mut self, t: &Token) {
        self.items.push(GroupItem::Token(t.clone()));
    }

    // Add repeat tokens comma separated
    pub fn add_comma_repeat_token(
        &mut self,
        s: &mut TokenStream,
        f: impl Fn(&mut TokenStream, &mut Self) -> Result<(), &'static str>,
        at_least_one: bool,
    ) -> Result<(), &'static str> {
        // first repeat token
        if let Err(e) = f(s, self) {
            if at_least_one {
                return Err(e);
            } else {
                return Ok(());
            }
        };
        // proceeding repeat tokens
        loop {
            let comma = s.symbol(',');
            match comma {
                Ok(c) => {
                    self.add_token(c);
                    f(s, self)?;
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    // Add a subgrouping
    pub fn add_grouping(&mut self, g: Grouping) {
        self.items.push(GroupItem::Grouping(g));
    }

    // Add repeat subgroupings
    pub fn add_repeat_grouping(
        &mut self,
        s: &mut TokenStream,
        f: impl Fn(&mut TokenStream) -> ParseResult,
    ) {
        loop {
            let g_res = f(s);
            match g_res {
                Ok(g) => self.add_grouping(g),
                Err(_) => break,
            }
        }
    }

    // Returns the XML string representation of the syntax tree
    pub fn as_xml(&self) -> String {
        format!(
            "<{}>{}</{}>",
            self.name,
            self.items
                .iter()
                .fold(String::new(), |acc, i| acc + &i.as_xml()),
            self.name
        )
    }

    // Returns only the tokens in a grouping
    pub fn tokens(&self) -> Vec<&Token> {
        self.items
            .iter()
            .map(|x| {
                if let GroupItem::Token(t) = x {
                    Some(t)
                } else {
                    None
                }
            })
            .filter(|x| matches!(x, Option::Some(_)))
            .map(|x| x.unwrap())
            .collect::<Vec<&Token>>()
    }

    // Returns only the sub-groupings in a grouping
    pub fn subgroupings(&self) -> Vec<&Grouping> {
        self.items
            .iter()
            .map(|x| {
                if let GroupItem::Grouping(g) = x {
                    Some(g)
                } else {
                    None
                }
            })
            .filter(|x| matches!(x, Option::Some(_)))
            .map(|x| x.unwrap())
            .collect::<Vec<&Grouping>>()
    }
}

#[derive(Debug, Clone)]
// Represents items in a grouping, can either be tokens or further substructures
pub enum GroupItem {
    Grouping(Grouping),
    Token(Token),
}

impl GroupItem {
    // Returns the XML string representation of the item
    fn as_xml(&self) -> String {
        match self {
            Self::Grouping(g) => g.as_xml(),
            Self::Token(t) => t.as_xml(),
        }
    }
}
