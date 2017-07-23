///! fff-lang
///!
///! syntax/root
///! root = { stmt }

use std::fmt;

use codemap::SymbolCollection;
use message::MessageCollection;
use lexical::Token;
use lexical::TokenStream;

use super::Statement;
use super::Formatter;
use super::ParseResult;
use super::ParseSession;
use super::ISyntaxItemParse;
use super::ISyntaxItemFormat;
use super::ISyntaxItemGrammar;

#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct SyntaxTree {
    pub items: Vec<Statement>,
}
impl ISyntaxItemFormat for SyntaxTree {
    fn format(&self, f: Formatter) -> String {
        format!("{}syntax-tree{}", 
            f.indent(),
            self.items.iter().fold(String::new(), |mut buf, item| { buf.push_str("\n"); buf.push_str(&f.apply1(item)); buf }))
    }
}
impl fmt::Debug for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.format(Formatter::default())) }
}
impl SyntaxTree {
    pub fn new_items(items: Vec<Statement>) -> SyntaxTree { SyntaxTree{ items } }
}
impl ISyntaxItemParse for SyntaxTree {
    type Target = SyntaxTree;

    fn parse(sess: &mut ParseSession) -> ParseResult<SyntaxTree> {

        let mut items = Vec::new();
        loop {
            if Statement::is_first_final(sess) {
                items.push(Statement::parse(sess)?);
            } else if sess.tk == &Token::EOF {
                break;
            } else {
                return sess.push_unexpect("if, while, for, var, const, expr");
            }
        }
        return Ok(SyntaxTree::new_items(items));
    }
}
impl SyntaxTree {
    pub fn new(tokens: &TokenStream, messages: &mut MessageCollection, symbols: &mut SymbolCollection) -> SyntaxTree {
        let mut sess = ParseSession::new(tokens, messages, symbols);
        match SyntaxTree::parse(&mut sess) {
            Ok(tree) => tree,
            Err(_) => SyntaxTree::new_items(Vec::new()),
        }
    }
}

#[cfg(test)] #[test]
fn syntax_tree_parse() {
    use std::fs::File;
    use std::io::Read;
    use super::TestInput;

    let mut index_file = File::open("../tests/syntax/index.txt").expect("cannot open index.txt");
    let mut test_cases = String::new();
    let _length = index_file.read_to_string(&mut test_cases).expect("cannot read index.txt");
    for line in test_cases.lines() {
        let src_path = "../tests/syntax/".to_owned() + line + "_src.ff";
        let mut src_file = File::open(&src_path).expect(&format!("cannot open src file {}", src_path));
        let mut src = String::new();
        let _length = src_file.read_to_string(&mut src).expect(&format!("cannot read src file {}", src_path));
        let result_path = "../tests/syntax/".to_owned() + line + "_result.txt";
        let mut result_file = File::open(&result_path).expect(&format!("cannot open result file {}", result_path));
        let mut expect = String::new();
        let _length = result_file.read_to_string(&mut expect).expect(&format!("cannot read result file {}", result_path));
        
        let result = TestInput::new(&src).apply::<SyntaxTree, _>();
        let actual = result.get_result().unwrap().format(Formatter::new(Some(result.get_source()), Some(result.get_symbols())));
        // let actual = SyntaxTree::with_test_str(&src).format(Formatter::default());
        if actual != expect {
            panic!("case '{}' failed, actual:\n`{}`\nexpect:\n`{}`", line, actual, expect)
        }
    }
}

// TODO: update format including format_with_codemap_symbols for these integration tests