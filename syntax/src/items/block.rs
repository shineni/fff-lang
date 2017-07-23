///! fff-lang
///!
///! syntax/block
///! Block = fLeftBrace [Statement]* fRightBrace

use std::fmt;

use codemap::Span;
use lexical::Token;
use lexical::Seperator;

use super::super::Statement;
use super::super::Formatter;
use super::super::ParseResult;
use super::super::ParseSession;
use super::super::ISyntaxItemParse;
use super::super::ISyntaxItemFormat;
use super::super::ISyntaxItemGrammar;

#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Block {
    pub items: Vec<Statement>,
    pub all_span: Span,
}
impl ISyntaxItemFormat for Block {
    fn format(&self, f: Formatter) -> String {
        format!("{}Block {}<{}>{}",
            f.indent(), if self.items.is_empty() { "(empty) " } else { "" }, f.span(self.all_span),
            self.items.iter().fold(String::new(), |mut buf, expr| { buf.push_str("\n"); buf.push_str(&f.apply1(expr)); buf })
        )
    }
}
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "\n{}", self.format(Formatter::default())) }
}
impl Block {

    pub fn new(all_span: Span, statements: Vec<Statement>) -> Block { Block{ all_span, items: statements } }
}
impl ISyntaxItemGrammar for Block {
    fn is_first_final(sess: &ParseSession) -> bool { sess.tk == &Token::Sep(Seperator::LeftBrace) }
}
impl ISyntaxItemParse for Block {
    type Target = Block;

    fn parse(sess: &mut ParseSession) -> ParseResult<Block> {

        let starting_strpos = sess.expect_sep(Seperator::LeftBrace)?;
        let mut items = Vec::new();
        loop {
            if let (&Token::Sep(Seperator::RightBrace), ref right_brace_strpos) = (sess.tk, sess.pos) {
                sess.move_next();
                return Ok(Block::new(starting_strpos.merge(right_brace_strpos), items));
            }
            items.push(Statement::parse(sess)?); 
        }
    }
}

#[cfg(test)] #[test]
fn block_parse() {
    use super::super::WithTestInput;
    
    assert_eq!{ Block::with_test_str("{}"),
        Block::new(make_span!(0, 1), vec![])
    }
}