///! fff-lang
///! 
///! lexical parser
///!
///! TokenStream::new(codechars, messages) for formal use
///! TokenStream::with_test_str(program) for test use

#[macro_use] extern crate util;
#[macro_use] extern crate messages as message; 
#[cfg_attr(test, macro_use)] extern crate codemap;

mod token_def;
mod buf_lexer;
mod token_stream;
mod v1lexer;
mod v2lexer;

pub use self::token_def::SeperatorKind;
pub use self::token_def::SeperatorCategory;
pub use self::token_def::KeywordKind;
pub use self::token_def::NumLitValue;
pub use self::token_def::LitValue;
pub use self::token_def::Token;
pub use self::token_stream::TokenStream;