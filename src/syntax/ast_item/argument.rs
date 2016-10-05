
// Argument -> Type Identifier

use message::MessageEmitter;
use lexical::BufLexer as Lexer;
use syntax::ast_item::ASTParser;
use syntax::Type;

#[derive(Debug, Eq, PartialEq)]
pub struct Argument {
    pub arg_type: Type,
    pub name: String,
}

impl ASTParser for Argument {
    
    fn parse(lexer: &mut Lexer, messages: &mut MessageEmitter) -> Option<Argument> {
        
        // enum State {
        //     ExpectType,
        //     ExpectIdentifier(Type),
        // }

        // loop {
        //     let bufv3 = lexer.next(messages);
        //     match State {
        //         State::ExpectType => {
                    
        //         }
        //     }
        // }
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn ast_argument_parse() {
        use message::MessageEmitter;
        use lexical::BufLexer as Lexer;
        use syntax::ast_item::ASTParser;
        use syntax::Type;
        use syntax::PrimitiveType;
        use super::Argument;

        macro_rules! test_case {
            ($program_slice: expr, $expect_type: expr, $expect_name: expr) => ({

                let messages = &mut MessageEmitter::new();
                let lexer = &mut Lexer::from_test($program_slice);
                assert_eq!(Argument::parse(lexer, messages), Some(Argument{ arg_type: $expect_type, name: $expect_name.to_owned() }));
            });
            ($program_slice: expr) => ({

                let messages = &mut MessageEmitter::new();
                let lexer = &mut Lexer::from_test($program_slice);
                assert_eq!(Argument::parse(lexer, messages), None);
            })
        }

        test_case!("");
        test_case!("i32 a", Type::Primitive(PrimitiveType::I32), "a");
        test_case!("[string] args", Type::Array(PrimitiveType::SMString), "args");
    }
}