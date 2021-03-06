use grammar::parse_tree::{Symbol, TypeRef};

#[test]
fn type_ref() {
    super::parse_type_ref("parser::Enum<'l,T>").unwrap();
}

#[test]
fn type_ref_tuple() {
    super::parse_type_ref("(X,Y)").unwrap();
}

#[test]
fn type_ref_special_case_for_id() {
    let x = super::parse_type_ref("X").unwrap();
    assert!(match x {
        TypeRef::Id(_) => true,
        _ => false
    });
}

#[test]
fn empty_grammar() {
    super::parse_grammar(r#"grammar { }"#).unwrap();
}

#[test]
fn grammar_param1() {
    super::parse_grammar(r#"grammar<T>(x: T) { }"#).unwrap();
}

#[test]
fn grammar_param2() {
    super::parse_grammar(
        r#"grammar<'a,T>(x: T, y: Vec<U>) where T: Copy, for<'a> &'a T: Baz { }"#
            ).unwrap();
}

#[test]
fn use_decls() {
    super::parse_grammar("grammar {
    use std::io;
    use std::io::{self, a, b};
    use std::baz::*;
    use std::this as that;
}").unwrap();
}

#[test]
fn alternative() {
    super::parse_alternative(r#"Alt => Bar;"#).unwrap();
}

#[test]
fn symbol() {
    super::parse_symbol(r#"Alt"#).unwrap();
}

#[test]
fn nonterminal0() {
    super::parse_grammar(r#"grammar { Expr = Alt; }"#).unwrap();
}

#[test]
fn paren() {
   super::parse_grammar(r#"grammar { Expr = (Alt); }"#).unwrap();
}

#[test]
fn paren_with_plus() {
    super::parse_grammar(r#"grammar { Expr = (Alt)+; }"#).unwrap();
}

#[test]
fn paren_with_plus_and_anon() {
    super::parse_grammar(r#"grammar { Expr = (~Alt)+; }"#).unwrap();
}

#[test]
fn named_choice() {
    super::parse_grammar(r#"grammar { Expr = n:Alt; }"#).unwrap();
}

#[test]
fn named_choice_plus() {
    super::parse_grammar(r#"grammar { Expr = ~Alt+; }"#).unwrap();
}

#[test]
fn token_expr() {
    super::parse_grammar(r#"grammar { token Expr where { "foo" => "bar"; }; }"#).unwrap();
}

#[test]
fn map1() {
    super::parse_grammar(
        r#"grammar { Expr = n:Alt+ => { { foo } }; }"#).unwrap();
}

#[test]
#[allow(non_snake_case)]
fn mapN() {
    super::parse_grammar(
        r#"grammar { Expr = { Bar => { Baz }; X n:Bar => { Y }; }; }"#).unwrap();
}

#[test]
fn macro_symbols() {
    super::parse_symbol(r#"Foo<Baz>"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz">"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+>"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+, "Balooga">"#).unwrap();
    super::parse_symbol(r#"Foo<"Baz"+, ("Balooga" Potato),>"#).unwrap();
}

#[test]
fn symbol_precedence() {
    // check that we parse this as choosing a X+
    let sym = super::parse_symbol(r#"~X+"#).unwrap();
    assert!(match sym {
        Symbol::Choose(..) => true,
        _ => false
    });

    let sym = super::parse_symbol(r#"n:X+"#).unwrap();
    assert!(match sym {
        Symbol::Name(..) => true,
        _ => false
    });
}

#[test]
fn symbol_choose_name() {
    // check that we can parse ~S and x:S but not both
    assert!(super::parse_symbol(r#"~x:X+"#).is_err());
}

#[test]
fn macro_nt() {
    super::parse_nonterminal(
        r#"Comma<E>: Vec<E> = v:(~E ",")* e:E? => v.into_iter().chain(e.into_iter()).collect();"#)
        .unwrap();
}

#[test]
fn cond_nt() {
    super::parse_nonterminal(
        "Foo<E> = {
           X if E == \"Bar\";
           X if E ~~ \"Bar\";
           X if E != \"Bar\";
           X if E !~ \"Bar\";
         };").unwrap();
}

