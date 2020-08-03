#![cfg(test)]

use crate::*;
use pipeline::*;
use serde_json::{Number, Value};
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

#[test]
fn test_run() {
    let tt = TreeTemplate::from_str(r#"Hello{{cat $0 "go" "dtop"}} "#).unwrap();
    let data = serde_json::Value::String("GOBBLE".to_string());
    let fm = func_man::default_func_man();
    let mut tm = temp_man::BasicTemps::new();
    let res = tt.run(&[&data], &mut tm, &fm).unwrap();
    assert_eq!(res, "HelloGOBBLEgodtop ");
}

#[test]
fn test_if() {
    let tt = TreeTemplate::from_str(
        r#"It's a {{if $1}}YES {{cat "from " $0}}{{else}}NO {{cat "from " $0}}{{/if}} too"#,
    )
    .unwrap();
    let fm = func_man::default_func_man();
    let mut tm = temp_man::BasicTemps::new();
    let res = tt.run(&[&"HIM", &true], &mut tm, &fm).unwrap();
    assert_eq!(res, "It's a YES from HIM too");
    let res2 = tt.run(&[&"HIM", &false], &mut tm, &fm).unwrap();
    assert_eq!(res2, "It's a NO from HIM too", "false is not false");
}

#[test]
fn test_for_array() {
    let tt =
        TreeTemplate::from_str(r#"Looping {{for k y in $0}}(k={{$k}},y={{$y}}){{/for}}"#).unwrap();
    let data = Value::Array(vec![
        Value::String("zero".to_string()),
        Value::String("one".to_string()),
        Value::String("two".to_string()),
        Value::String("three".to_string()),
    ]);
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&data], &mut tm, &fm).unwrap();
    assert_eq!(
        res,
        "Looping (k=0,y=zero)(k=1,y=one)(k=2,y=two)(k=3,y=three)"
    );
}

#[test]
fn test_first_sel() {
    let tt = TreeTemplate::from_str(
        r#"{{select $0 "MOO", "NOO"}} is\    
        \ {{select $1 'null' $10 $2 $3}} {{first_valid $10 'null' $3}} "#,
    )
    .unwrap();

    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt
        .run(&[&true, &2, &"HELLO", &"GOODBYE"], &mut tm, &fm)
        .unwrap();
    assert_eq!(res, "MOO is HELLO GOODBYE ");
}
#[test]
fn test_for_json_own_method() {
    let tt = TreeTemplate::from_str(r#"JRES = {{to_json $0}}"#).unwrap();
    let data = Value::Array(vec![
        Value::String("zero".to_string()),
        Value::Number(Number::from(1)),
        Value::String("two".to_string()),
        Value::String("three".to_string()),
    ]);
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&data], &mut tm, &fm).unwrap();
    assert_eq!(res, r#"JRES = ["zero",1,"two","three"]"#);
}

#[test]
fn test_json_math() {
    let tt =
        TreeTemplate::from_str(r#"3*({{$0}}+{{$1}}+{{$2}})={{mul 3 (add $0 $1 $2)}}"#).unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&3, &5.2, &100], &mut tm, &fm).unwrap();
    assert_eq!(res, "3*(3+5.2+100)=324.6");
}

#[test]
fn test_var_part() {
    let tt = TreeTemplate::from_str(r#"{{$0.0}}+{{$0.1}}+{{$0.2}}"#).unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt
        .run(
            &[&Value::Array(vec![
                Value::from(3),
                Value::from(5.2),
                Value::from(100),
            ])],
            &mut tm,
            &fm,
        )
        .unwrap();

    assert_eq!(res, "3+5.2+100");
}

#[test]
fn test_at_can_be_used_in_params() {
    let tt = TreeTemplate::from_str(r#"{{@cat "Food" @ "noobs"}} {{$0}} {{/cat}}!!"#).unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&"is for"], &mut tm, &fm).unwrap();

    assert_eq!(res, "Food is for noobs!!");
}
