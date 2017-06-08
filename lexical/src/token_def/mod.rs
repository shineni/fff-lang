///! fff-lang
///!
///! lexical/token

use std::fmt;
use codemap::SymbolID;

mod keyword;
mod seperator;
mod lit_value;

pub use self::keyword::KeywordKind;
pub use self::seperator::SeperatorKind;
pub use self::seperator::SeperatorCategory;
pub use self::lit_value::LitValue;
pub use self::lit_value::NumLitValue;

/// Lexical token
#[derive(Eq, PartialEq)]  
pub enum Token {
    EOF,
    Lit(LitValue),
    Ident(SymbolID),
    Label(SymbolID),
    Sep(SeperatorKind),
    Keyword(KeywordKind),
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::EOF => write!(f, "EOF"), 
            &Token::Lit(ref lit) => write!(f, "{:?}", lit),
            &Token::Ident(ref sid) => write!(f, "Ident {:?}", sid),  // Ident #1
            &Token::Label(ref sid) => write!(f, "Lable @{:?}", sid), // Label @#2
            &Token::Sep(ref sep) => write!(f, "Seperator {:?}", sep),
            &Token::Keyword(ref kw) => write!(f, "Keyword {:?}", kw),
        }
    }
}
