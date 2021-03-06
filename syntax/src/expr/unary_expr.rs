///! fff-lang
///!
///! syntax/unary_expr
///! unary_expr = { unary_operator } postfix_expr

use std::fmt;

use codemap::Span;
use lexical::Token;
use lexical::Seperator;
use lexical::SeperatorCategory;

use super::Expr;
use super::PostfixExpr;

use super::super::Formatter;
use super::super::ParseResult;
use super::super::ParseSession;
use super::super::ISyntaxParse;
use super::super::ISyntaxFormat;
use super::super::ISyntaxGrammar;

#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct UnaryExpr {
    pub base: Box<Expr>, 
    pub operator: Seperator, 
    pub operator_span: Span,
    pub all_span: Span,
}
impl ISyntaxFormat for UnaryExpr {
    fn format(&self, f: Formatter) -> String {
        f.indent().header_text_or("unary-expr").space().span(self.all_span).endl()
            .indent1().lit("\"").debug(&self.operator).lit("\"").space().span(self.operator_span).endl()
            .apply1(self.base.as_ref())
            .finish()
    }
}
impl fmt::Debug for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "\n{}", self.format(Formatter::empty())) }
}
impl From<UnaryExpr> for Expr {
    fn from(unary_expr: UnaryExpr) -> Expr { Expr::Unary(unary_expr) }
}
impl UnaryExpr {

    pub fn new<T: Into<Expr>>(operator: Seperator, operator_span: Span, base: T) -> UnaryExpr {
        let base = base.into();
        UnaryExpr{
            all_span: operator_span.merge(&base.get_all_span()),
            base: Box::new(base),
            operator, operator_span,
        }
    }
}
impl ISyntaxGrammar for UnaryExpr {
    fn matches_first(tokens: &[&Token]) -> bool { 
        match tokens[0] {
            &Token::Sep(ref sep) if sep.is_category(SeperatorCategory::Unary) => true,
            _ => false,
        }
    }
}
impl ISyntaxParse for UnaryExpr {
    type Output = Expr;

    fn parse(sess: &mut ParseSession) -> ParseResult<Expr> {
        
        let mut op_spans = Vec::new();
        loop {
            match sess.try_expect_sep_cat(SeperatorCategory::Unary) {
                Some((sep, sep_span)) => op_spans.push((sep, sep_span)),
                None => {
                    let base = PostfixExpr::parse(sess)?;
                    return Ok(op_spans.into_iter().rev().fold(base, |base, (op, span)| { Expr::Unary(UnaryExpr::new(op, span, base)) }));
                }
            }
        }
    }
}

#[cfg(test)] #[test]
fn unary_expr_parse() {
    use lexical::LitValue;
    use super::LitExpr;
    use super::super::WithTestInput;
    
    assert_eq!{ UnaryExpr::with_test_str("1"), 
        Expr::Lit(LitExpr::new(LitValue::from(1), make_span!(0, 0))) 
    }

    assert_eq!{ UnaryExpr::with_test_str("!~!1"),
        Expr::Unary(UnaryExpr::new(
            Seperator::LogicalNot, make_span!(0, 0),
            Expr::Unary(UnaryExpr::new(
                Seperator::BitNot, make_span!(1, 1),            
                Expr::Unary(UnaryExpr::new(
                    Seperator::LogicalNot, make_span!(2, 2),
                    Expr::Lit(LitExpr::new(LitValue::from(1), make_span!(3, 3))),
                ))
            ))
        ))
    }
}