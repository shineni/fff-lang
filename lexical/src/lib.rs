///! fff-lang
///! 
///! lexical parser
///! TokenStream::new(source, messages) for formal use
///! TokenStream::with_test_str(input) for test use

#[cfg_attr(test, macro_use)] extern crate messages as message; 
#[cfg_attr(test, macro_use)] extern crate util;
#[cfg_attr(test, macro_use)] extern crate codemap;

#[cfg_attr(test, macro_use)] mod token_def;
mod token_buf;
mod v1lexer;
mod v2lexer;
mod token_stream;

use self::token_buf::ILexer;
use self::token_buf::BufLexer;
use self::token_buf::ParseSession;

pub use self::token_def::Seperator;
pub use self::token_def::SeperatorCategory;
pub use self::token_def::Keyword;
pub use self::token_def::NumLitValue;
pub use self::token_def::StrLitValue;
pub use self::token_def::FormatStrLitSegment;
pub use self::token_def::LitValue;
pub use self::token_def::Token;
pub use self::token_stream::TokenStream;
