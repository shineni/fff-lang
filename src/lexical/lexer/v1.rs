
// Level1 parser
// input v0
// remove line comment
// report block comment as OtherChar ' '
// find string literal with only '"' escaped
// string literal is allowed to cross line, line end is regarded as \n
// raw string literal supported, `r'C:\\abc'` or `R"C:\\abc"`

use common::Position;
use common::StringPosition;
use lexical::symbol_type::string_literal::StringLiteral;
use lexical::symbol_type::char_literal::CharLiteral;
#[cfg(test)]
#[derive(Clone, Eq, PartialEq)]
pub enum V1Token {
    StringLiteral { inner: StringLiteral },
    CharLiteral { inner: CharLiteral },
    OtherChar { raw: char, pos: Position },
}
#[cfg(not(test))]
#[derive(Clone)]
pub enum V1Token {
    StringLiteral { inner: StringLiteral },
    CharLiteral { inner: CharLiteral },
    OtherChar { raw: char, pos: Position },
}

#[cfg(test)]
use std::fmt;
#[cfg(test)]
impl fmt::Debug for V1Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            V1Token::StringLiteral { ref inner} => {
                write!(f, "{:?}", inner)
            }
            V1Token::CharLiteral{ ref inner } => {
                write!(f, "{:?}", inner)
            }
            V1Token::OtherChar { ref raw, ref pos } => {
                write!(f, "Char {:?} at {:?}", raw, pos)
            }
        }
    }
}

use lexical::lexer::v0::V0Lexer;
use lexical::lexer::v0::BufV0Lexer;
pub struct V1Lexer {
    v0: BufV0Lexer,
}
impl From<String> for V1Lexer {

    fn from(content: String) -> V1Lexer {
        V1Lexer { 
            v0: BufV0Lexer::from(V0Lexer::from(content)),
        }
    }
}

use lexical::ILexer;
use lexical::lexer::v0::V0Token;
use lexical::lexer::v0::BufV0Token;
use lexical::message::Message;
use lexical::message::MessageEmitter;
impl V1Lexer {
    pub fn position(&self) -> Position { self.v0.inner().position() }
}

impl ILexer<V1Token> for V1Lexer {

    // input v0, output stringliteral or otherchar without comment
    fn next(&mut self, messages: &mut MessageEmitter) -> Option<V1Token> {
        // First there is quote, and anything inside is regarded as string literal, include `\n` as real `\n`
        // and then outside of quote pair there is comments, anything inside comment, // and /n, or /* and */ is regarded as comment

        #[derive(Debug)]
        enum State {
            Nothing,
            InStringLiteral { 
                raw: String, 
                start_pos: Position, 
                last_escape_quote_pos: Option<Position>, 
                has_failed: bool 
            },
            InRawStringLiteral { 
                raw: String, 
                start_pos: Position 
            },
            InLineComment,
            InBlockComment { 
                start_pos: Position 
            },
            InCharLiteral { 
                raw: String, 
                start_pos: Position 
            },
        }

        let mut state = State::Nothing;
        loop {
            let bufv0 = self.v0.next(messages);
            match state {
                State::Nothing => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token{ ch: '/', pos: _1 }, next: Some(V0Token{ ch: '/', pos: _2 }) }) => {
                            self.v0.skip1(messages);
                            state = State::InLineComment;                                      // State conversion 1: in nothing, meet //
                        }
                        Some(BufV0Token{ token: V0Token{ ch: '/', pos }, next: Some(V0Token{ ch: '*', pos: _1 }) }) => {
                            state = State::InBlockComment { start_pos: pos };                  // State conversion 2: in nothing, meet /*
                        }
                        Some(BufV0Token{ token: V0Token{ ch: '"', pos }, next: _1 }) => {
                            state = State::InStringLiteral {                                   // State conversion 3: in nothing, meet "
                                raw: String::new(), 
                                start_pos: pos,
                                last_escape_quote_pos: None, 
                                has_failed: false
                            };
                        }
                        Some(BufV0Token{ token: V0Token{ ch: '\'', pos }, next: _1 }) => {     // C24: in nothing, meet '
                            state = State::InCharLiteral {
                                raw: String::new(),
                                start_pos: pos,
                            }
                        }
                        Some(BufV0Token { token: V0Token { ch: 'r', pos }, next: Some(V0Token { ch: '"', pos: _1 }) })
                            | Some(BufV0Token { token: V0Token { ch: 'R', pos }, next: Some(V0Token { ch: '"', pos: _1 }) }) => {
                            self.v0.skip1(messages);                                           // State conversion 4: in nothing, meet r" or R"
                            state = State::InRawStringLiteral { raw: String::new(), start_pos: pos };
                        }
                        Some(BufV0Token{ token: V0Token{ ch, pos }, next: _1 }) => {
                            return Some(V1Token::OtherChar{ raw: ch, pos: pos });              // State conversion 5: in nothing, meet other, return
                        }
                        None => { return None; }                                               // State conversion 6: in nothing, meet EOF, return 
                    }
                }
                State::InBlockComment { start_pos } => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token { ch: '*', pos: _1 }, next: Some(V0Token{ ch: '/', pos: _2 }) }) => {
                            self.v0.skip1(messages);
                            return Some(V1Token::OtherChar{ raw: ' ', pos: start_pos });      // State conversion 7: in block, meet */, return
                        }
                        Some(_) => {
                            state = State::InBlockComment{ start_pos: start_pos };             // State conversion 8: in block, continue block
                        }
                        None => {
                            messages.push(Message::UnexpectedEndofFileInBlockComment { block_start: start_pos, eof_pos: self.v0.inner().position() });
                            return None;                                                       // State conversion 9: in block, meet EOF, emit error, return
                        }
                    }
                }
                State::InLineComment => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token { ch: '\n', pos }, next: _1 }) => {
                            return Some(V1Token::OtherChar { raw: '\n', pos: pos });           // State conversion 10: in line, meet \n, return
                        }
                        Some(_) => {
                            state = State::InLineComment;                                      // State conversion 11: in line, continue line
                        }
                        None => {
                            return None;                                                       // State conversion 12: in line, meet EOF, return
                        }
                    }
                }
                State::InStringLiteral { mut raw, start_pos, last_escape_quote_pos, has_failed } => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token { ch: '\\', pos: slash_pos }, next: Some(V0Token{ ch: next_ch, pos: _1 }) }) => {
                            match next_ch {
                                '"' => {
                                    // record escaped \" here to be error hint
                                    raw.push('"');
                                    state = State::InStringLiteral {                           // State conversion 13: in string, meet \", store it 
                                        raw: raw, 
                                        start_pos: start_pos, 
                                        last_escape_quote_pos: Some(slash_pos), 
                                        has_failed: has_failed,
                                    };
                                    self.v0.skip1(messages);
                                }
                                next_ch @ 'n' | next_ch @ '\\' | next_ch @ 't' | next_ch @ 'r' | next_ch @ '0'  => {
                                    raw.push(match next_ch {'n' => '\n', 'r' => '\r', 't' => '\t', '\\' => '\\', '0' => '\0', ch => ch}); 
                                    state = State::InStringLiteral {                           // State conversion 14: in string, meet \\nrt, escape
                                        raw: raw, 
                                        start_pos: start_pos,
                                        last_escape_quote_pos: last_escape_quote_pos,
                                        has_failed: has_failed,
                                    };
                                    self.v0.skip1(messages);
                                }
                                'u' => { 
                                    // Leave \u{} forward
                                    raw.push('\\');
                                    raw.push('u'); 
                                    state = State::InStringLiteral {                            // State conversion 15: in string, meet \u, leave forward 
                                        raw: raw, 
                                        start_pos: start_pos,
                                        last_escape_quote_pos: last_escape_quote_pos, 
                                        has_failed: has_failed,
                                    };
                                    self.v0.skip1(messages);
                                }
                                other => {
                                    messages.push(Message::UnrecogonizedEscapeCharInStringLiteral {
                                        literal_start: start_pos, 
                                        unrecogonize_pos: slash_pos, 
                                        unrecogonize_escape: other });
                                    // here actually is meaning less                            // State conversion 16: in string, meet \other, emit error, continue
                                    state = State::InStringLiteral { 
                                        raw: raw, start_pos: start_pos, 
                                        last_escape_quote_pos: last_escape_quote_pos, has_failed: true };
                                    self.v0.skip1(messages);
                                }
                            }
                        }
                        Some(BufV0Token{ token: V0Token { ch: '\\', pos }, next: None }) => {   // State conversion 17: in string, meet \EOF, continue
                            state = State::InStringLiteral { raw: raw, start_pos: pos, last_escape_quote_pos: last_escape_quote_pos, has_failed: true };
                        }
                        Some(BufV0Token{ token: V0Token { ch: '"', pos }, next: _1 }) => {      // State conversion 18: in string, meet ", finish, return
                            // String finished
                            return Some(V1Token::StringLiteral { inner: StringLiteral::new(raw, StringPosition{ start_pos: start_pos, end_pos: pos }, false, has_failed) });
                        }
                        Some(BufV0Token{ token: V0Token { ch, pos: _1 }, next: _2 }) => {
                            // Normal in string
                            raw.push(ch);                                                       // State conversion 19: in string, meet other, push, continue
                            state = State::InStringLiteral { raw: raw, start_pos: start_pos, last_escape_quote_pos: last_escape_quote_pos, has_failed: has_failed };
                        }
                        None => {
                            messages.push(Message::UnexpectedEndofFileInStringLiteral {         // State conversion 20: in string, meet EOF, emit error, return 
                                literal_start: start_pos, 
                                eof_pos: self.position(), 
                                hint_escaped_quote_pos: last_escape_quote_pos });
                            return None;
                        }
                    }
                }
                State::InRawStringLiteral { mut raw, start_pos } => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token { ch: '"', pos }, next: _2 }) => {      // State conversion 21: in raw string, meet ", finish, return
                            return Some(V1Token::StringLiteral { inner: StringLiteral::new(raw, StringPosition { start_pos: start_pos, end_pos: pos }, true, false) });
                        }
                        Some(BufV0Token{ token: V0Token { ch, pos: _1 }, next: _2 }) => {       // State conversion 22: in raw string, meet other, continue
                            raw.push(ch);
                            state = State::InRawStringLiteral{ raw: raw, start_pos: start_pos  };
                        }
                        None => {
                            messages.push(Message::UnexpectedEndofFileInStringLiteral {         // State conversion 23: in raw string, meet EOF, emit error, return  
                                literal_start: start_pos, 
                                eof_pos: self.position(),
                                hint_escaped_quote_pos: None
                            });
                            return None;
                        }
                    }
                }
                State::InCharLiteral { mut raw, start_pos } => {
                    match bufv0 {
                        Some(BufV0Token{ token: V0Token{ ch: '\\', pos: _1 }, next: Some(V0Token{ ch: '\'', pos: _2 }) }) => {
                            raw.push('\'');                                                     // C25: in char literal, meet \', continue
                            state = State::InCharLiteral{ raw: raw, start_pos: start_pos };
                            self.v0.skip1(messages);
                        }
                        Some(BufV0Token{ token: V0Token{ ch: '\'', pos }, next: _1 }) => {      // C26: in cahr literal, meet ', return
                            return Some(V1Token::CharLiteral{ inner: CharLiteral::from(raw, StringPosition::from((start_pos, pos))) });
                        }
                        Some(BufV0Token{ token: V0Token{ ch, pos: _2 }, next: _1 }) => {        // C27, in char literal, meet other
                            raw.push(ch);
                            state = State::InCharLiteral{ raw: raw, start_pos: start_pos };
                        }
                        None => {
                            messages.push(Message::UnexpectedEndofFileInCharLiteral {           // C28, in char literal, meet EOF, emit error, return
                                literal_start: start_pos, 
                                eof_pos: self.position(),
                            });
                            return None;
                        }
                    }
                }
            }
        }
    }
}

use lexical::lexer::buf_lexer::BufToken;
use lexical::lexer::buf_lexer::BufLexer;
pub type BufV1Token = BufToken<V1Token>;
pub type BufV1Lexer = BufLexer<V1Lexer, V1Token>;

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]

    use super::V1Lexer;
    use common::Position;
    use common::StringPosition;   
    use lexical::ILexer;
    use lexical::message::MessageEmitter;
    use lexical::symbol_type::string_literal::StringLiteral;
    
    // C[\d]+[r]{0-1}, nth state conversion, have 'r' means return, s means skipped
    // C20r1, C20r2, C23r1, C23r2, r1 means with escape quote hint, r2 without, ATTENTION: no C23r2
    // 5, 3, 19, 18, 2; 8, 7, 4, 22, 21; 11, 10, 14, 15, 9; 12, 20, 23, 1, 6; 13, 16, 17

    const program1: &'static str = concat!(
        "abc\"def\"ghi/*jkl*/\n",              // C5r, C5r, C5r, C3, C19, C19, C19, C18r, C5r, C5r, C5r, C2, s, C8, C8, C8, C7r, s, C5r,
        "mr\"\\u\\n\\r\\a\\bc\\\"mno//pqr\n",  // C5r, C4, s, C22, C22, C22, C22, C22, C22, C22, C22, C22, C22, C22, C22, C21r, C5r, C5r, C5r, C1, s, C11, C11, C11, C10r, 
        "stuv\"\"wx/**/y//\n",                 // C5r, C5r, C5r, C5r, C3, C18r, C5r, C5r, C2, s, C7r, s, C5r, C1, s, C10r,
        "\"\\t\\n\\r\\\\\\u///**///\n",        // C3, C14, s, C14, s, C14, s, C14, s, C14, s, C15, s, C19, C19, C19, C19, C19, C19, C19, C19, C18r, C5r,
        "//\"/**/\"");                         // C1, s, C11, C11, C11, C11, C11, C11, C10r, C6
    const program2: &'static str = "abc//def";                 // C12
    const program3: &'static str = "abc/*def";                 // C9
    const program4: &'static str = "abc\"123";                 // C20r1
    const program5: &'static str = "abc\"123\\\"456";          // C13, C20r2
    const program6: &'static str = "abcr\"123";                // C23r1
    const program7: &'static str = "abcR\"123\\\"456";         // C13, C23r2
    const program8: &'static str = "abc\"def\\a\\b\\c";        // C16
    const program9: &'static str = "abc\"\\";                  // C17
    const programa: &'static str = "abc\"123\\a\"";

    #[test]
    fn v1_test1() {
        use super::V1Token;

        macro_rules! test_case {
            ($program: expr, $($expect: expr, )*) => (
                let mut v1lexer = V1Lexer::from($program.to_owned());
                let mut messages = MessageEmitter::new();
                let mut v1s = Vec::new();
                loop {
                    match v1lexer.next(&mut messages) {
                        Some(v1) => v1s.push(v1),
                        None => break,
                    }
                }

                assert_eq!(v1s, vec![$($expect, )*]);
                if !messages.is_empty() {
                    perrorln!("Messages for {}:", stringify!($program));
                    perror!("{:?}", messages);
                }
            )
        }
        macro_rules! tch {
            ($ch: expr, $row: expr, $col: expr) => (V1Token::OtherChar{ raw: $ch, pos: Position { row: $row, col: $col } })
        }
        macro_rules! tstring {
            ($val: expr, $row1: expr, $col1: expr, $row2: expr, $col2: expr, $is_raw: expr, $has_fail: expr) => 
                (V1Token::StringLiteral { inner: StringLiteral::new($val.to_owned(), 
                    StringPosition { 
                        start_pos: Position { row: $row1, col: $col1 },
                        end_pos: Position { row: $row2, col: $col2 } },
                    $is_raw,
                    $has_fail) })
        }

        // Start cases
        test_case!(program1,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            tstring!("def", 1, 4, 1, 8, false, false),
            tch!('g', 1, 9),
            tch!('h', 1, 10), 
            tch!('i', 1, 11),
            tch!(' ', 1, 12),
            tch!('\n', 1, 19),
            tch!('m', 2, 1),
            tstring!("\\u\\n\\r\\a\\bc\\", 2, 2, 2, 16, true, false),
            tch!('m', 2, 17),
            tch!('n', 2, 18),
            tch!('o', 2, 19),
            tch!('\n', 2, 25),
            tch!('s', 3, 1),
            tch!('t', 3, 2),
            tch!('u', 3, 3),
            tch!('v', 3, 4),
            tstring!("", 3, 5, 3, 6, false, false),
            tch!('w', 3, 7), 
            tch!('x', 3, 8),
            tch!(' ', 3, 9), 
            tch!('y', 3, 13),
            tch!('\n', 3, 16),
            tstring!("\t\n\r\\\\u///**///\n//", 4, 1, 5, 3, false, false),
            tch!(' ', 5, 4),
        );

        test_case!(program2, 
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
        );
        test_case!(program3,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // tcomment!(1, 4), // Not returned but UnexpectedEndofFileInBlockComment
        );
        test_case!(program4,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // tstring!(),  // Not returned but UnexpectedEndofFileInStringLiteral, r2
        );
        test_case!(program5,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // tstring!(),   // Not returned but UnexpectedEndofFileInStringLiteral, r2
        );
        test_case!(program6,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // tstring!(),  // Not returned but UnexpectedEndofFileInStringLiteral, r2
        );
        test_case!(program7,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            tstring!("123\\", 1, 4, 1, 10, true, false),   // Returned but UnexpectedEndofFileInStringLiteral, r2
            tch!('4', 1, 11),
            tch!('5', 1, 12), 
            tch!('6', 1, 13),
        );
        test_case!(program8,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // No more return
        );
        test_case!(program9,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            // No more return
        );
        test_case!(programa,
            tch!('a', 1, 1),
            tch!('b', 1, 2),
            tch!('c', 1, 3),
            tstring!("123", 1, 4, 1, 10, false, true),
        );
    }

    #[test]
    fn v1_buf() {
        use super::BufV1Lexer;

        macro_rules! test_case {
            ($program: expr) => (

                let mut bufv1 = BufV1Lexer::from(V1Lexer::from($program.to_owned()));
                let mut messages = MessageEmitter::new();
                loop {
                    match bufv1.next(&mut messages) {
                        Some(v1) => perrorln!("{:?}", v1),
                        None => break,
                    }
                }
                perror!("{:?}", messages);
            )
        }

        test_case!(program1);
    }
}