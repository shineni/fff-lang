///! fff-lang
///!
///! syntax, abstract syntax tree types and generation

#[macro_use] extern crate messages as message;
#[cfg_attr(test, macro_use)] extern crate util;
#[cfg_attr(test, macro_use)] extern crate codemap;
extern crate lexical;

mod traits;
mod syntax_tree;
mod statement;
mod expr;
mod items;
mod parse_sess;
mod error_strings;

pub use self::items::FnParam;
pub use self::items::FnDef;
pub use self::items::TypeUse;
pub use self::items::Block;
pub use self::items::LabelDef;
pub use self::items::NameSegment;
pub use self::items::Name;
pub use self::expr::LitExpr;
pub use self::expr::IdentExpr;
pub use self::expr::Expr;
pub use self::expr::BinaryExpr;
pub use self::expr::UnaryExpr;
pub use self::expr::FnCallExpr;
pub use self::expr::IndexCallExpr;
pub use self::expr::MemberAccessExpr;
pub use self::expr::ParenExpr;
pub use self::expr::TupleDef;
pub use self::expr::ExprList;
pub use self::expr::ArrayDef;
pub use self::statement::Statement;
pub use self::statement::VarDeclStatement;
pub use self::statement::ReturnStatement;
pub use self::statement::BreakStatement;
pub use self::statement::ContinueStatement;
pub use self::statement::ExprStatement;
pub use self::statement::LoopStatement;
pub use self::statement::WhileStatement;
pub use self::statement::ForStatement;
pub use self::statement::IfConditionBody;
pub use self::statement::IfStatement;
pub use self::syntax_tree::SyntaxTree;

use self::parse_sess::ParseSession;
use self::parse_sess::ParseResult;
use self::traits::ISyntaxItemFormat;
use self::traits::ISyntaxItemGrammar;
pub use self::traits::ISyntaxItemParse; // for semantic/traits
pub use self::traits::ISyntaxItemWithStr;

// TODO: 
// finish these structs test
// finish primary expr _format, _parse and _errors tests
// replace proper place by IdentExpr and ExprList
// seperate PostfixExpr members, decide too remove member function call
// merge postfix expr and primary expr enums into expr enum
// abort IdentExpr to use Name, check name should be single segment at many where