///! fff-lang
///!
///! syntax/block_stmt
///! BlockStatement for explicit block definition in block and allow block label
///! BlockStatement = [LabelDef] Block

use std::fmt;

use codemap::Span;
use lexical::Token;
use lexical::Seperator;

use super::super::Block;
use super::super::LabelDef;
use super::super::Formatter;
use super::super::ParseResult;
use super::super::ParseSession;
use super::super::ISyntaxItemParse;
use super::super::ISyntaxItemFormat;
use super::super::ISyntaxItemGrammar;

#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct BlockStatement {
    pub name: Option<LabelDef>,
    pub body: Block,
    pub all_span: Span,
}
impl ISyntaxItemFormat for BlockStatement {
    fn format(&self, f: Formatter) -> String {
        match self.name {
            Some(ref name) => format!("{}BlockStmt <{}>\n{}\n{}", 
                f.indent(), f.span(self.all_span),
                f.apply1(name),
                f.apply1(&self.body)
            ),
            None => format!("{}BlockStmt <{}>\n{}", 
                f.indent(), f.span(self.all_span),
                f.apply1(&self.body)
            )
        }
    }
}
impl fmt::Debug for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.format(Formatter::default())) }
}
impl BlockStatement {
    
    pub fn new_no_label(body: Block) -> BlockStatement { BlockStatement{ all_span: body.all_span, body, name: None } }
    pub fn new_with_label(name: LabelDef, body: Block) -> BlockStatement { 
        BlockStatement { all_span: name.all_span.merge(&body.all_span), body, name: Some(name) } 
    }

    fn new(name: Option<LabelDef>, body: Block) -> BlockStatement { 
        BlockStatement{ 
            all_span: match name { Some(ref name) => name.all_span.merge(&body.all_span), None => body.all_span },
            body, name
        } 
    }
}
impl ISyntaxItemGrammar for BlockStatement {
    fn is_first_final(sess: &ParseSession) -> bool { 
        match (sess.tk, sess.nextnext_tk) {
            (&Token::Label(_), &Token::Sep(Seperator::LeftBrace)) 
            | (&Token::Sep(Seperator::LeftBrace), _) => true,
            _ => false,
        }
    }
}
impl ISyntaxItemParse for BlockStatement {
    type Target = BlockStatement;

    fn parse(sess: &mut ParseSession) -> ParseResult<BlockStatement> {
    
        let maybe_name = LabelDef::try_parse(sess)?;
        let body = Block::parse(sess)?;
        return Ok(BlockStatement::new(maybe_name, body));
    }
}

#[cfg(test)] #[test]
fn block_stmt_parse() {
    use super::super::WithTestInput;

    assert_eq!{ BlockStatement::with_test_str("{}"), BlockStatement::new_no_label(Block::new(make_span!(0, 1), vec![])) }
    assert_eq!{ BlockStatement::with_test_str("@: {}"), 
        BlockStatement::new_with_label(
            LabelDef::new(make_id!(1), make_span!(0, 1)),
            Block::new(make_span!(3, 4), vec![])
        )
    }
}