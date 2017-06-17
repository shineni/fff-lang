///! fff-lang
///!
///! syntax/priority level proxy
///! they are here because they are dispatcher not containing data
///! primary_expr = ident_expr | lit_expr | unit_lit | paren_expr | tuple_def | array_def
///! postfix_expr = expr { ( member_access | fn_call | indexer_call ) }

use lexical::Token;
use lexical::KeywordKind;

use super::Expr;
use super::LitExpr;
use super::IdentExpr;
use super::TupleDef;
use super::ArrayDef;
use super::FnCallExpr;
use super::IndexCallExpr;
use super::MemberAccessExpr;

use super::super::ParseSession;
use super::super::ParseResult;
use super::super::ISyntaxItemParse;
use super::super::ISyntaxItemGrammar;

pub struct PrimaryExpr;
impl ISyntaxItemParse for PrimaryExpr {
    type Target = Expr;
    
    fn parse(sess: &mut ParseSession) -> ParseResult<Expr> {
        #[cfg(feature = "trace_primary_expr_parse")]
        macro_rules! trace { ($($arg:tt)*) => ({ print!("[PrimaryExpr: {}]", line!()); println!($($arg)*); }) }
        #[cfg(not(feature = "trace_primary_expr_parse"))]
        macro_rules! trace { ($($arg:tt)*) => () }

        trace!("start parsing, current token: {:?}", sess.tk);

        if LitExpr::is_first_final(sess) {
            return LitExpr::parse(sess);
        } else if IdentExpr::is_first_final(sess) {
            return IdentExpr::parse(sess);
        } else if TupleDef::is_first_final(sess) {
            return TupleDef::parse(sess);
        } else if ArrayDef::is_first_final(sess) {
            return ArrayDef::parse(sess);
        }

        if let (&Token::Keyword(KeywordKind::This), this_span) = (sess.tk, sess.pos) {
            sess.move_next();
            return Ok(Expr::Ident(IdentExpr::new(sess.symbols.intern_str("this"), this_span)));
        } else {
            return sess.push_unexpect("primary expr");
        }
    }
}

pub struct PostfixExpr;
impl ISyntaxItemParse for PostfixExpr {
    type Target = Expr;

    fn parse(sess: &mut ParseSession) -> ParseResult<Expr> {   
        #[cfg(feature = "trace_postfix_expr_parse")]
        macro_rules! trace { ($($arg:tt)*) => ({ perror!("    [PostfixExpr:{}] ", line!()); perrorln!($($arg)*); }) }
        #[cfg(not(feature = "trace_postfix_expr_parse"))]
        macro_rules! trace { ($($arg:tt)*) => () }

        let mut current_retval = PrimaryExpr::parse(sess)?;
        trace!("parsed primary, current is {:?}", current_retval);

        loop {
            if MemberAccessExpr::is_first_final(sess) {
                let mut postfix = MemberAccessExpr::parse(sess)?;
                postfix.all_span = current_retval.get_all_span().merge(&postfix.name.span);
                postfix.base = Box::new(current_retval);
                current_retval = Expr::MemberAccess(postfix);
            } else if FnCallExpr::is_first_final(sess) {
                let mut postfix = FnCallExpr::parse(sess)?;
                postfix.all_span = current_retval.get_all_span().merge(&postfix.paren_span);
                postfix.base = Box::new(current_retval);
                current_retval = Expr::FnCall(postfix);
            } else if IndexCallExpr::is_first_final(sess) {
                let mut postfix = IndexCallExpr::parse(sess)?;
                postfix.all_span = current_retval.get_all_span().merge(&postfix.bracket_span);
                postfix.base = Box::new(current_retval);
                current_retval = Expr::IndexCall(postfix);
            } else {
                break;
            }
        }

        trace!("parsing postfix finished, get retval: {:?}", current_retval);
        return Ok(current_retval);
    }
}

#[cfg(test)] #[test]
fn primary_expr_parse() {
    use codemap::Span;
    use codemap::SymbolCollection;
    use lexical::LitValue;
    use super::ParenExpr;
    use super::super::ISyntaxItemWithStr;

    // this is the loop of tokens.nth(current) is left bracket does not cover everything and infinite loop is here
    // update 2017/6/17: this was a bug, but I forget detail
    assert_eq!{ PrimaryExpr::with_test_str("[a]"),  
        Expr::Array(ArrayDef::new(make_span!(0, 2), vec![
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(1, 1)))
        ]))
    }

    //                                        0        1         2         3         4
    //                                        01234567890123456789012345678901234567890123456
    assert_eq!{ PrimaryExpr::with_test_input("(463857, IEfN, atau8M, [fNAE, ((cAeJN4)), nHg])", 
        //                  1       2         3       4         5
        &mut make_symbols!["IEfN", "atau8M", "fNAE", "cAeJN4", "nHg"]), 
        Expr::Tuple(TupleDef::new(make_span!(0, 46), vec![
            Expr::Lit(LitExpr::new(LitValue::from(463857), make_span!(1, 6))),
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(9, 12))),
            Expr::Ident(IdentExpr::new(make_id!(2), make_span!(15, 20))),
            Expr::Array(ArrayDef::new(make_span!(23, 45), vec![
                Expr::Ident(IdentExpr::new(make_id!(3), make_span!(24, 27))),
                Expr::Paren(ParenExpr::new(make_span!(30, 39), 
                    Expr::Paren(ParenExpr::new(make_span!(31, 38), 
                        Expr::Ident(IdentExpr::new(make_id!(4), make_span!(32, 37)))
                    ))
                )),
                Expr::Ident(IdentExpr::new(make_id!(5), make_span!(42, 44)))
            ]))
        ]))
    }

    assert_eq!{ PrimaryExpr::with_test_str("10363"), 
        Expr::Lit(LitExpr::new(LitValue::from(10363), make_span!(0, 4)))
    }

    assert_eq!{ PrimaryExpr::with_test_str(
    //   0         1         2         3         4         5         6         7         8         9         0         1         2         3       
    //   01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567 8901234567890123456789012345678901234
        "[(0x7E), FFGqfJe, I4, [(m7A, (41, ([(jL, rAn, K0FgLc7h, true), C, w]), (J3cEFDG, d, (j8h))), (), \neIuArjF), 400, 0o535147505, 0xDB747]]"),
        Expr::Array(ArrayDef::new(make_span!(0, 134), vec![
            Expr::Paren(ParenExpr::new(make_span!(1, 6), 
                Expr::Lit(LitExpr::new(LitValue::from(0x7E), make_span!(2, 5)))
            )),
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(9, 15))),
            Expr::Ident(IdentExpr::new(make_id!(2), make_span!(18, 19))), 
            Expr::Array(ArrayDef::new(make_span!(22, 133), vec![
                Expr::Tuple(TupleDef::new(make_span!(23, 105), vec![
                    Expr::Ident(IdentExpr::new(make_id!(3), make_span!(24, 26))),
                    Expr::Tuple(TupleDef::new(make_span!(29, 90), vec![
                        Expr::Lit(LitExpr::new(LitValue::from(41), make_span!(30, 31))),
                        Expr::Paren(ParenExpr::new(make_span!(34, 68), 
                            Expr::Array(ArrayDef::new(make_span!(35, 67), vec![
                                Expr::Tuple(TupleDef::new(make_span!(36, 60), vec![
                                    Expr::Ident(IdentExpr::new(make_id!(4), make_span!(37, 38))), 
                                    Expr::Ident(IdentExpr::new(make_id!(5), make_span!(41, 43))),
                                    Expr::Ident(IdentExpr::new(make_id!(6), make_span!(46, 53))),
                                    Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(56, 59)))
                                ])),
                                Expr::Ident(IdentExpr::new(make_id!(7), make_span!(63, 63))),
                                Expr::Ident(IdentExpr::new(make_id!(8), make_span!(66, 66))),
                            ]))
                        )),
                        Expr::Tuple(TupleDef::new(make_span!(71, 89), vec![
                            Expr::Ident(IdentExpr::new(make_id!(9), make_span!(72, 78))),
                            Expr::Ident(IdentExpr::new(make_id!(10), make_span!(81, 81))),
                            Expr::Paren(ParenExpr::new(make_span!(84, 88), 
                                Expr::Ident(IdentExpr::new(make_id!(11), make_span!(85, 87)))
                            ))
                        ]))
                    ])),
                    Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(93, 94))),
                    Expr::Ident(IdentExpr::new(make_id!(12), make_span!(98, 104)))
                ])),
                Expr::Lit(LitExpr::new(LitValue::from(400), make_span!(108, 110))),
                Expr::Lit(LitExpr::new(LitValue::from(0o535147505), make_span!(113, 123))),
                Expr::Lit(LitExpr::new(LitValue::from(0xDB747), make_span!(126, 132)))
            ]))
        ]))
    }

    assert_eq!{ PrimaryExpr::with_test_str("CMDoF"), Expr::Ident(IdentExpr::new(make_id!(1), make_span!(0, 4))) }
    assert_eq!{ PrimaryExpr::with_test_str("false"), Expr::Lit(LitExpr::new(LitValue::from(false), make_span!(0, 4))) }

    
    //                                      0        1         2         3         4         5         6          7          8         9         A
    //                                      12345678901234567890123456789012345678901234567890123456789012345678901 234 5678901234567890123456789012
    assert_eq!{ PrimaryExpr::with_test_str("[uy6, 4373577, [(q, AJBN0n, MDEgKh5,), KG, (NsL, ((), D, false, d, ), \"H=\"), true, ((vvB3, true, 5))]]"), 
        Expr::Array(ArrayDef::new(make_span!(0, 101), vec![
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(1, 3))),
            Expr::Lit(LitExpr::new(LitValue::from(4373577), make_span!(6, 12))),
            Expr::Array(ArrayDef::new(make_span!(15, 100), vec![
                Expr::Tuple(TupleDef::new(make_span!(16, 36), vec![
                    Expr::Ident(IdentExpr::new(make_id!(2), make_span!(17, 17))),
                    Expr::Ident(IdentExpr::new(make_id!(3), make_span!(20, 25))), 
                    Expr::Ident(IdentExpr::new(make_id!(4), make_span!(28, 34)))
                ])), 
                Expr::Ident(IdentExpr::new(make_id!(5), make_span!(39, 40))),
                Expr::Tuple(TupleDef::new(make_span!(43, 74), vec![
                    Expr::Ident(IdentExpr::new(make_id!(6), make_span!(44, 46))),
                    Expr::Tuple(TupleDef::new(make_span!(49, 67), vec![
                        Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(50, 51))),
                        Expr::Ident(IdentExpr::new(make_id!(7), make_span!(54, 54))),
                        Expr::Lit(LitExpr::new(LitValue::from(false), make_span!(57, 61))),
                        Expr::Ident(IdentExpr::new(make_id!(8), make_span!(64, 64))),
                    ])),
                    Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(9)), make_span!(70, 73)))
                ])),
                Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(77, 80))),
                Expr::Paren(ParenExpr::new(make_span!(83, 99), 
                    Expr::Tuple(TupleDef::new(make_span!(84, 98), vec![
                        Expr::Ident(IdentExpr::new(make_id!(10), make_span!(85, 88))),
                        Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(91, 94))),
                        Expr::Lit(LitExpr::new(LitValue::from(5), make_span!(97, 97)))
                    ]))
                ))
            ]))
        ]))
    }

    assert_eq!{ PrimaryExpr::with_test_str("(() )"), 
        Expr::Paren(ParenExpr::new(make_span!(0, 4), LitExpr::new(LitValue::Unit, make_span!(1, 2))))
    }
    assert_eq!{ PrimaryExpr::with_test_str("((),)"), 
        Expr::Tuple(TupleDef::new(make_span!(0, 4), vec![Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(1, 2)))]))
    }

    assert_eq!{ PrimaryExpr::with_test_str("(\"o5\")"), 
        Expr::Paren(ParenExpr::new(make_span!(0, 5), 
            Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(1)), make_span!(1, 4)))
        ))
    }

    //                                      0        1         2        
    //                                      1234567890123456789012345678
    assert_eq!{ PrimaryExpr::with_test_str("(nn, ([false,true]), 183455)"),
        Expr::Tuple(TupleDef::new(make_span!(0, 27), vec![
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(1, 2))),
            Expr::Paren(ParenExpr::new(make_span!(5, 18), 
                Expr::Array(ArrayDef::new(make_span!(6, 17), vec![
                    Expr::Lit(LitExpr::new(LitValue::from(false), make_span!(7, 11))),
                    Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(13, 16)))
                ]))
            )),
            Expr::Lit(LitExpr::new(LitValue::from(183455), make_span!(21, 26)))
        ]))
    }
    
    //                                      0        1         2         3         4         5         6         7       
    //                                      123456789012345678901234567890123456789012345678901234567890123456789012345678
    assert_eq!{ PrimaryExpr::with_test_str("((true, (mO, [(q5k),a], (((KttG))), (K5DJ, r, ())), (McsaEdfdfalse,)), rIOKt,)"),
        Expr::Tuple(TupleDef::new(make_span!(0, 77), vec![
            Expr::Tuple(TupleDef::new(make_span!(1, 68), vec![
                Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(2, 5))),
                Expr::Tuple(TupleDef::new(make_span!(8, 49), vec![
                    Expr::Ident(IdentExpr::new(make_id!(1), make_span!(9, 10))),
                    Expr::Array(ArrayDef::new(make_span!(13, 21), vec![
                        Expr::Paren(ParenExpr::new(make_span!(14, 18), 
                            Expr::Ident(IdentExpr::new(make_id!(2), make_span!(15, 17)))
                        )),
                        Expr::Ident(IdentExpr::new(make_id!(3), make_span!(20, 20)))
                    ])),
                    Expr::Paren(ParenExpr::new(make_span!(24, 33), 
                        Expr::Paren(ParenExpr::new(make_span!(25, 32), 
                            Expr::Paren(ParenExpr::new(make_span!(26, 31), 
                                Expr::Ident(IdentExpr::new(make_id!(4), make_span!(27, 30)))
                            ))
                        ))
                    )),
                    Expr::Tuple(TupleDef::new(make_span!(36, 48), vec![
                        Expr::Ident(IdentExpr::new(make_id!(5), make_span!(37, 40))), 
                        Expr::Ident(IdentExpr::new(make_id!(6), make_span!(43, 43))),
                        Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(46, 47)))
                    ])),
                ])),
                Expr::Tuple(TupleDef::new(make_span!(52, 67), vec![
                    Expr::Ident(IdentExpr::new(make_id!(7), make_span!(53, 65)))
                ]))
            ])),
            Expr::Ident(IdentExpr::new(make_id!(8), make_span!(71, 75)))
        ]))
    }

    //                                      0          1         2      
    //                                      12 345 67890123456789012
    assert_eq!{ PrimaryExpr::with_test_str("[\"il\", 0o52u32, sO04n]"),
        Expr::Array(ArrayDef::new(make_span!(0, 21), vec![
            Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(1)), make_span!(1, 4))),
            Expr::Lit(LitExpr::new(LitValue::from(0o52u32), make_span!(7, 13))), 
            Expr::Ident(IdentExpr::new(make_id!(2), make_span!(16, 20)))
        ]))
    }
    //                                      12345678
    assert_eq!{ PrimaryExpr::with_test_str("['f',()]"), 
        Expr::Array(ArrayDef::new(make_span!(0, 7), vec![
            Expr::Lit(LitExpr::new(LitValue::from('f'), make_span!(1, 3))),
            Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(5, 6)))
        ]))
    }
    assert_eq!{ PrimaryExpr::with_test_str("[]"), Expr::Array(ArrayDef::new(make_span!(0, 1), vec![])) }

    //                                      0        1           2         3         4      
    //                                      12345 678901 234567890123456789012345678901234
    assert_eq!{ PrimaryExpr::with_test_str("[8, \"@=?GF\", 87f32, 1340323.74f64, FKOxAvx5]"),
        Expr::Array(ArrayDef::new(make_span!(0, 43), vec![
            Expr::Lit(LitExpr::new(LitValue::from(8), make_span!(1, 1))),
            Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(1)), make_span!(4, 10))), 
            Expr::Lit(LitExpr::new(LitValue::from(87f32), make_span!(13, 17))),
            Expr::Lit(LitExpr::new(LitValue::from(1340323.74f64), make_span!(20, 32))),
            Expr::Ident(IdentExpr::new(make_id!(2), make_span!(35, 42)))
        ]))
    }

    //                                        0        1         2         3         4         5         6     
    //                                        123456789012345678901234567890123456789012345678901234567890123
    assert_eq!{ PrimaryExpr::with_test_str(r#"  [[dnr4, lGFd3yL, tJ], ['\\', p, (xGaBwiL,), DE], true, aB8aE]"#),
        Expr::Array(ArrayDef::new(make_span!(2, 62), vec![
            Expr::Array(ArrayDef::new(make_span!(3, 21), vec![
                Expr::Ident(IdentExpr::new(make_id!(1), make_span!(4, 7))),
                Expr::Ident(IdentExpr::new(make_id!(2), make_span!(10, 16))),
                Expr::Ident(IdentExpr::new(make_id!(3), make_span!(19, 20)))
            ])),
            Expr::Array(ArrayDef::new(make_span!(24, 48), vec![
                Expr::Lit(LitExpr::new(LitValue::from('\\'), make_span!(25, 28))), 
                Expr::Ident(IdentExpr::new(make_id!(4), make_span!(31, 31))),
                Expr::Tuple(TupleDef::new(make_span!(34, 43), vec![
                    Expr::Ident(IdentExpr::new(make_id!(5), make_span!(35, 41)))
                ])),
                Expr::Ident(IdentExpr::new(make_id!(6), make_span!(46, 47)))
            ])),
            Expr::Lit(LitExpr::new(LitValue::from(true), make_span!(51, 54))),
            Expr::Ident(IdentExpr::new(make_id!(7), make_span!(57, 61)))
        ]))
    } 

    // Previous manual tests
    //                                      0        1           2          3         4         5           6
    //                                      12345678901234 5678 9012 3456789012345678901234567890123 456789 0123456
    assert_eq!{ PrimaryExpr::with_test_input("[abc, 123u32, \"456\", '\\u0065', false, (), (a), (abc, \"hello\", ), ]",
        &mut make_symbols!["abc", "456", "hello", "a"]),
        Expr::Array(ArrayDef::new(make_span!(0, 65), vec![
            Expr::Ident(IdentExpr::new(make_id!(1), make_span!(1, 3))),
            Expr::Lit(LitExpr::new(LitValue::from(123u32), make_span!(6, 11))),
            Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(2)), make_span!(14, 18))),
            Expr::Lit(LitExpr::new(LitValue::from('\u{0065}'), make_span!(21, 28))),
            Expr::Lit(LitExpr::new(LitValue::from(false), make_span!(31, 35))),
            Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(38, 39))),
            Expr::Paren(ParenExpr::new(make_span!(42, 44), 
                Expr::Ident(IdentExpr::new(make_id!(4), make_span!(43, 43)))
            )),
            Expr::Tuple(TupleDef::new(make_span!(47, 62), vec![
                Expr::Ident(IdentExpr::new(make_id!(1), make_span!(48, 50))),
                Expr::Lit(LitExpr::new(LitValue::new_str_lit(make_id!(3)), make_span!(53, 59))),
            ]))
        ]))
    }       

    assert_eq!{ PrimaryExpr::with_test_str("(                             )"), 
        Expr::Lit(LitExpr::new(LitValue::Unit, make_span!(0, 30)))
    }
}

#[cfg(test)] #[test]
fn primary_expr_errors() {
    use codemap::Span;
    use message::Message;
    use message::MessageCollection;
    use super::super::error_strings;
    use super::super::ISyntaxItemWithStr;

    assert_eq!{ PrimaryExpr::with_test_str_ret_messages("(,)"), (
        Some(Expr::Tuple(TupleDef::new(make_span!(0, 2), vec![]))),
        make_messages![
            Message::new_by_str(error_strings::UnexpectedSingleComma, vec![(make_span!(0, 2), error_strings::TupleDefHere)])
        ]
    )}
}

#[cfg(test)] #[test]
fn postfix_expr_format() {
    use super::super::ISyntaxItemFormat;
    use super::super::ISyntaxItemWithStr;

    macro_rules! test_case {
        ($left: expr, $right: expr) => {
            if $left != $right {
                let left_owned = $left.to_owned();
                let left_lines = left_owned.lines();
                let right_lines = $right.lines();
                for (index, (left_line, right_line)) in left_lines.zip(right_lines).enumerate() {
                    if left_line != right_line {
                        panic!("assertion failed at index {}\nleft: {}\nright: {}", index, $left, $right);
                    }
                }
                panic!("assertion failed, but cannot detected by compare each line\nleft: {}\nright: {}", $left, $right);
            }
        }
    }

    // Attention that this source code line's LF is also the string literal (test oracle)'s LF
    //                                                     0         1         2         3         4         5        
    //                                                     0123456789012345678901234567890123456789012345678901234567
    test_case!(format!("\n{}", PostfixExpr::with_test_str("a.b(c, d, e).f(g, h, i,)(u,).j[k].l().m[n, o, p][r, s, t,]").format(0)), r##"
IndexerCall <<0>0-57>
  IndexerCall <<0>0-47>
    MemberAccess <<0>0-38>
      FnCall <<0>0-36>
        MemberAccess <<0>0-34>
          IndexerCall <<0>0-32>
            MemberAccess <<0>0-29>
              FnCall <<0>0-27>
                FnCall <<0>0-23>
                  MemberAccess <<0>0-13>
                    FnCall <<0>0-11>
                      MemberAccess <<0>0-2>
                        Ident #1 <<0>0-0>
                        dot <<0>1-1>
                        Ident #2 <<0>2-2>
                      paren <<0>3-11>
                      Ident #3 <<0>4-4>
                      Ident #4 <<0>7-7>
                      Ident #5 <<0>10-10>
                    dot <<0>12-12>
                    Ident #6 <<0>13-13>
                  paren <<0>14-23>
                  Ident #7 <<0>15-15>
                  Ident #8 <<0>18-18>
                  Ident #9 <<0>21-21>
                paren <<0>24-27>
                Ident #10 <<0>25-25>
              dot <<0>28-28>
              Ident #11 <<0>29-29>
            bracket <<0>30-32>
            Ident #12 <<0>31-31>
          dot <<0>33-33>
          Ident #13 <<0>34-34>
        paren <<0>35-36>
        (empty)
      dot <<0>37-37>
      Ident #14 <<0>38-38>
    bracket <<0>39-47>
    Ident #15 <<0>40-40>
    Ident #16 <<0>43-43>
    Ident #17 <<0>46-46>
  bracket <<0>48-57>
  Ident #18 <<0>49-49>
  Ident #19 <<0>52-52>
  Ident #20 <<0>55-55>"##
    );
}

#[cfg(test)] #[test]
fn postfix_expr_parse() {
    use codemap::Span;
    use super::IdentExpr;
    use super::super::ISyntaxItemWithStr;

    //                                      0        1         2         3         4         5     
    // plain                                0123456789012345678901234567890123456789012345678901234567
    assert_eq!{ PostfixExpr::with_test_str("a.b(c, d, e).f(g, h, i,)(u,).j[k].l().m[n, o, p][r, s, t,]"),
        Expr::IndexCall(IndexCallExpr::new(
            IndexCallExpr::new(
                MemberAccessExpr::new(
                    FnCallExpr::new(
                        MemberAccessExpr::new(
                            IndexCallExpr::new(
                                MemberAccessExpr::new(
                                    FnCallExpr::new(
                                        FnCallExpr::new(
                                            MemberAccessExpr::new(
                                                FnCallExpr::new(
                                                    MemberAccessExpr::new(
                                                        IdentExpr::new(make_id!(1), make_span!(0, 0)),
                                                        make_span!(1, 1),
                                                        IdentExpr::new(make_id!(2), make_span!(2, 2))
                                                    ), 
                                                    make_span!(3, 11), vec![
                                                        Expr::Ident(IdentExpr::new(make_id!(3), make_span!(4, 4))),
                                                        Expr::Ident(IdentExpr::new(make_id!(4), make_span!(7, 7))),
                                                        Expr::Ident(IdentExpr::new(make_id!(5), make_span!(10, 10))),
                                                    ]
                                                ),
                                                make_span!(12, 12),
                                                IdentExpr::new(make_id!(6), make_span!(13, 13))
                                            ),
                                            make_span!(14, 23), vec![
                                                Expr::Ident(IdentExpr::new(make_id!(7), make_span!(15, 15))),
                                                Expr::Ident(IdentExpr::new(make_id!(8), make_span!(18, 18))),
                                                Expr::Ident(IdentExpr::new(make_id!(9), make_span!(21, 21))),
                                            ]
                                        ),
                                        make_span!(24, 27), vec![
                                            Expr::Ident(IdentExpr::new(make_id!(10), make_span!(25, 25)))
                                        ]
                                    ),
                                    make_span!(28, 28),
                                    IdentExpr::new(make_id!(11), make_span!(29, 29))
                                ),
                                make_span!(30, 32), vec![
                                    Expr::Ident(IdentExpr::new(make_id!(12), make_span!(31, 31)))
                                ]
                            ),
                            make_span!(33, 33),
                            IdentExpr::new(make_id!(13), make_span!(34, 34))
                        ),
                        make_span!(35, 36),
                        vec![]
                    ),
                    make_span!(37, 37),
                    IdentExpr::new(make_id!(14), make_span!(38, 38))
                ),
                make_span!(39, 47), vec![
                    Expr::Ident(IdentExpr::new(make_id!(15), make_span!(40, 40))),
                    Expr::Ident(IdentExpr::new(make_id!(16), make_span!(43, 43))),
                    Expr::Ident(IdentExpr::new(make_id!(17), make_span!(46, 46)))
                ]
            ),
            make_span!(48, 57), vec![
                Expr::Ident(IdentExpr::new(make_id!(18), make_span!(49, 49))),
                Expr::Ident(IdentExpr::new(make_id!(19), make_span!(52, 52))),
                Expr::Ident(IdentExpr::new(make_id!(20), make_span!(55, 55)))
            ]
        ))
    }
}

#[cfg(test)] #[test]
fn postfix_expr_errors() {
    use codemap::Span;
    use message::Message;
    use message::MessageCollection;
    use super::super::error_strings;
    use super::super::ISyntaxItemWithStr;
    
    assert_eq!{ PostfixExpr::with_test_str_ret_messages("a[]"), (
        Some(Expr::IndexCall(IndexCallExpr::new(
            IdentExpr::new(make_id!(1), make_span!(0, 0)), 
            make_span!(1, 2), vec![]
        ))), 
        make_messages![
            Message::new_by_str(error_strings::EmptyIndexCall, vec![(make_span!(1, 2), error_strings::IndexCallHere)])
        ],
    )}
    
    assert_eq!{ PostfixExpr::with_test_str_ret_messages("a[, ]"), (
        Some(Expr::IndexCall(IndexCallExpr::new(
            IdentExpr::new(make_id!(1), make_span!(0, 0)), 
            make_span!(1, 4), vec![]
        ))), 
        make_messages![
            Message::new_by_str(error_strings::EmptyIndexCall, vec![(make_span!(1, 4), error_strings::IndexCallHere)])
        ],
    )}
    
    assert_eq!{ PostfixExpr::with_test_str_ret_messages("a(, )"), (
        Some(Expr::FnCall(FnCallExpr::new(
            IdentExpr::new(make_id!(1), make_span!(0, 0)),
            make_span!(1, 4), vec![]
        ))),
        make_messages![
            Message::new_by_str(error_strings::UnexpectedSingleComma, vec![(make_span!(1, 4), error_strings::FnCallHere)])
        ],
    )}
}