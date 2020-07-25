#![cfg(test)]
use crate::*;
use pipeline::*;
use std::str::FromStr;
use template::*;

#[test]
fn test_simple_parser() {
    let p = FlatTemplate::from_str("{{let x = 3}}").unwrap();

    assert_eq!(
        p,
        FlatTemplate {
            v: vec![FlatItem::Let(vec![(
                "x".to_string(),
                Pipeline::Lit("3".to_string())
            )])]
        }
    );
}

#[test]
fn test_fancy() {
    let tt = TreeTemplate::from_str(
        r#"hello{{ dothing $h 5}}More words{{if $0}}YES{{else}}NO{{/if}}LastBit"#,
    )
    .unwrap();
    assert_eq!(tt.v[0], TreeItem::String("hello".to_string()));
    assert_eq!(tt.v[2], TreeItem::String("More words".to_string()));
    assert_eq!(tt.v[4], TreeItem::String("LastBit".to_string()));
    match &tt.v[3] {
        TreeItem::If { cond, yes, no } => {
            assert_eq!(cond, &Pipeline::Var(vec![VarPart::Num(0)]));
        }
        e => panic!("Expected 'if' got {:?}", e),
    };
}
