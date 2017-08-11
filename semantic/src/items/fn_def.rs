///! fff-lang
///!
///! semantic/fn_def

use codemap::SymbolID;
use syntax;

use super::TypeUse;
use super::Block;
use super::super::FromSession;
use super::super::SharedDefScope;
use super::super::ISemanticAnalyze;

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct FnParam {
    pub name: SymbolID,
    pub typeuse: TypeUse,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct FnDef {
    pub name: SymbolID,
    pub params: Vec<FnParam>,
    pub rettype: Option<TypeUse>,
    pub body: Block,
    pub this_scope: SharedDefScope,
}
impl ISemanticAnalyze for FnDef {

    type SyntaxItem = syntax::FnDef;

    fn from_syntax(node: syntax::FnDef, sess: FromSession) -> FnDef {
        let this_sess = sess.sub_with_symbol(node.name);
        FnDef{
            name: node.name,
            params: node.params.into_iter().map(|param| {
                FnParam{
                    name: param.name,
                    typeuse: TypeUse::from_syntax(param.decltype, this_sess.clone_scope()),
                }
            }).collect(),
            rettype: node.ret_type.map(|ty| TypeUse::from_syntax(ty, this_sess.clone_scope())),
            body: Block::from_syntax(node.body, this_sess.clone_scope()),
            this_scope: this_sess.into_scope(),
        }
    }
}
