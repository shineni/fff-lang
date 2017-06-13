///! fff-lang
///!
///! syntax/name, currently is
///! Name = fIdent [ fNamespaceSep fIdent ]*
// future may support something like `to_string::<i32>(a)`

use std::fmt;

use codemap::Span;
use codemap::SymbolID;
use lexical::Token;
use lexical::SeperatorKind;

use super::super::ISyntaxItemFormat;
use super::super::ISyntaxItemGrammar;
use super::super::ISyntaxItemParse;
use super::super::ParseSession;
use super::super::ParseResult;

#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct NameSegment{
    pub value: SymbolID,
    pub span: Span,
}
impl NameSegment {
    fn new(value: SymbolID, span: Span) -> NameSegment { NameSegment{ value, span } }
}
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Name {
    pub segments: Vec<NameSegment>,
    pub all_span: Span,
}
impl ISyntaxItemFormat for Name {
    fn format(&self, indent: u32) -> String {
        format!("{}Name <{:?}>{}", 
            Name::indent_str(indent), self.all_span,
            self.segments.iter().fold(String::new(), |mut buf, segment| { 
                buf.push_str(&format!("\n{}Ident {:?} <{:?}>", Name::indent_str(indent + 1), segment.value, segment.span));
                buf 
            }),
        )
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.format(0)) }
}
impl Name {
    fn new(all_span: Span, segments: Vec<NameSegment>) -> Name { Name{ all_span, segments } }
}
impl ISyntaxItemGrammar for Name {
    fn is_first_final(sess: &ParseSession) -> bool { if let &Token::Ident(_) = sess.tk { true } else { false } }
}
impl ISyntaxItemParse for Name {

    fn parse(sess: &mut ParseSession) -> ParseResult<Name> {
        
        let mut segments = Vec::new();
        let starting_span = sess.pos;

        let (first_ident, first_ident_span) = sess.expect_ident()?;
        segments.push(NameSegment::new(first_ident, first_ident_span));
        let mut ending_span = first_ident_span;

        loop {
            if let &Token::Sep(SeperatorKind::NamespaceSeperator) = sess.tk {
                sess.move_next();
                let (ident, ident_span) = sess.expect_ident()?;
                segments.push(NameSegment::new(ident, ident_span));
                ending_span = ident_span;
            } else {
                break;
            }
        }
        return Ok(Name::new(starting_span.merge(&ending_span), segments));
    }
}

#[cfg(test)] #[test]
fn name_parse() {
    use codemap::SymbolCollection;
    use super::super::ISyntaxItemWithStr;

    assert_eq!{ Name::with_test_str("hello"), 
        Name::new(make_span!(0, 4), vec![
            NameSegment::new(make_id!(1), make_span!(0, 4))
        ]) 
    }
    //                                 0        1         2         3         4
    //                                 01234567890123456789012345678901234567890
    assert_eq!{ Name::with_test_input("std::network::wlan::native::GetWLANHandle", 
        &mut make_symbols!["std", "network", "wlan", "native", "GetWLANHandle"]),
        Name::new(make_span!(0, 40), vec![
            NameSegment::new(make_id!(1), make_span!(0, 2)), 
            NameSegment::new(make_id!(2), make_span!(5, 11)),
            NameSegment::new(make_id!(3), make_span!(14, 17)),
            NameSegment::new(make_id!(4), make_span!(20, 25)),
            NameSegment::new(make_id!(5), make_span!(28, 40)),
        ])
    }
}