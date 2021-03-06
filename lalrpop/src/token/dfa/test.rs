use token::dfa::{self, Ambiguity, DFA, NFAIndex, Precedence};
use token::dfa::interpret::interpret;
use token::re;

fn dfa(inputs: &[(&str, Precedence)]) -> Result<DFA,Ambiguity> {
    let regexs: Result<Vec<_>, _> = inputs.iter().map(|&(s, _)| re::parse_regex(s)).collect();
    let regexs = regexs.unwrap();
    let precedences: Vec<_> = inputs.iter().map(|&(_, p)| p).collect();
    dfa::build_dfa(&regexs, &precedences)
}

const P1: Precedence = Precedence(1);
const P0: Precedence = Precedence(0);

#[test]
fn tokenizer() {
    let dfa = dfa(&[
        /* 0 */ (r#"class"#, P1),
        /* 1 */ (r#"[a-zA-Z_][a-zA-Z0-9_]*"#, P0),
        /* 2 */ (r#"[0-9]+"#, P0),
        /* 3 */ (r#" +"#, P0),
        /* 4 */ (r#">>"#, P0),
        /* 5 */ (r#">"#, P0),
        ]).unwrap();

    assert_eq!(interpret(&dfa, "class Foo"), Some((NFAIndex(0), "class")));
    assert_eq!(interpret(&dfa, "classz Foo"), Some((NFAIndex(1), "classz")));
    assert_eq!(interpret(&dfa, "123"), Some((NFAIndex(2), "123")));
    assert_eq!(interpret(&dfa, "  classz Foo"), Some((NFAIndex(3), "  ")));
    assert_eq!(interpret(&dfa, ">"), Some((NFAIndex(5), ">")));
    assert_eq!(interpret(&dfa, ">>"), Some((NFAIndex(4), ">>")));
}

#[test]
fn ambiguous_regex() {
    // here the keyword and the regex have same precedence, so we have
    // an ambiguity
    assert!(dfa(&[(r#"class"#, P0),
                  (r#"[a-zA-Z_][a-zA-Z0-9_]*"#, P0)]).is_err());
}
