#![cfg(test)]

use crate::*;
use pipeline::*;
use serde_json::Value;
use std::str::FromStr;
use temp_man::*;
use template::*;

#[test]
fn can_parse_simple_template() {
    let p = FlatTemplate::from_str("{{let x = 3}}").unwrap();

    assert_eq!(
        p,
        FlatTemplate {
            v: vec![FlatItem::Let(vec![(
                "x".to_string(),
                Pipeline::Lit(TData::UInt(3))
            )])]
        }
    );
}

#[test]
fn let_assign_vars() {
    let p = TreeTemplate::from_str("{{let x = 3\ny=\"poo\"}}{{$x}} + {{$y}}").unwrap();
    let fm = func_man::default_func_man();
    let res = p.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, "3 + poo");
}

#[test]
fn cat_can_print_all_members() {
    let tt = TreeTemplate::from_str(r#"Hello{{cat $0 "go" 3}} "#).unwrap();
    let data = serde_json::Value::String("GOBBLE".to_string());
    let fm = func_man::default_func_man();
    let mut tm = temp_man::BasicTemps::new();
    let res = tt.run(&[&data], &mut tm, &fm).unwrap();
    assert_eq!(res, "HelloGOBBLEgo3 ");
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
fn first_gets_first_valid() {
    let tt = TreeTemplate::from_str(
        r#"{{select $0 "MOO" "NOO"}} is\    
        \ {{select $1 null $10 $2 $3}} {{first $10 null $3}} "#,
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
fn json_math() {
    let tt =
        TreeTemplate::from_str(r#"3*({{$0}}+{{$1}}+{{$2}})={{mul 3 (add $0 $1 $2)}}"#).unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&3, &5.2, &100], &mut tm, &fm).unwrap();
    assert_eq!(res, "3*(3+5.2+100)=324.6");
}

#[test]
fn can_access_arrays() {
    let tt = TreeTemplate::from_str(r#"{{$0.0}}+{{.1}}+{{$0.2}}"#).unwrap();
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
fn at_can_be_used_in_params() {
    let tt = TreeTemplate::from_str(r#"{{@cat "Food" @ "noobs"}} {{$0}} {{/cat}}!!"#).unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&"is for"], &mut tm, &fm).unwrap();

    assert_eq!(res, "Food is for noobs!!");
}

#[test]
fn can_call_defined_templates() {
    let tt =
        TreeTemplate::from_str(r#"{{define good}}{{$0}} > {{$1}}{{/define}}{{run $good 3 4}}"#)
            .unwrap();
    let mut tm = temp_man::BasicTemps::new();
    let fm = func_man::default_func_man();
    let res = tt.run(&[&"is for"], &mut tm, &fm).unwrap();
    assert_eq!(res, "3 > 4");
}

#[test]
fn sorting() {
    let fm = func_man::default_func_man();
    let tt =
        TreeTemplate::from_str(r#"{{#some comment#}}{{let a = (sort [ 10 , 4, 11, 15])}}{{$a}}"#)
            .unwrap();
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, "[4,10,11,15]");
}

#[test]
fn sort_on() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"\
        {{let a = (sort_on [
            {"name":"dave","age":5},
            {"name":"pete","age":10},
            {"name":"matt","age":16},
            {"name":"dave","age":100},
            {"name":"matt","age":12}
        ] "name" "age")}}{{for k v in $a}}{{$v.name}}={{$v.age}},{{/for}}"#,
    )
    .unwrap();
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, "dave=5,dave=100,matt=12,matt=16,pete=10,");
}

#[test]
fn return_non_str() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"\
        {{define cow}}
        {{return (add $0 $1)}} 
        {{/define}}\
        {{run $cow 3 5}}"#,
    )
    .unwrap();
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, "8");
}

#[test]
fn global_def() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"\
        {{global cow}}
        {{return (add $0 $1)}} 
        {{/global}}\
        {{cow 3 5}}"#,
    )
    .unwrap();
    let res = tt.run(&[], &mut BasicTemps::new(), &fm).unwrap();
    assert_eq!(res, "8");
}

#[test]
fn filter() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"\
        {{let a = (filter [
            {"name":"dave","age":5},
            {"name":"pete","age":10},
            {"name":"matt","age":16},
            {"name":"dave","age":100},
            {"name":"matt","age":12}
        ] "str_contains $0.name \"a\"")}}{{for k v in $a}}{{$v.name}}={{$v.age}},{{/for}}"#,
    )
    .unwrap();
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, "dave=5,matt=16,dave=100,matt=12,");
}

#[test]
fn can_handle_gaps() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"{{let a = 3 -}}
            I say a ="
        {{- $a -}}
            ""#,
    )
    .unwrap();
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"I say a ="3""#);
}

#[test]
fn switch_can() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"
        {{- switch $0 $1}}
        {{- case 3 2 -}}
            It's 3 and 2
        {{- case 5 -}}            
            Something Something 5 Something
        {{- case -}}
            Default case
        {{- /switch -}}
            "#,
    )
    .unwrap();
    let res = tt.run(&[&3, &2], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"It's 3 and 2"#);
    let res = tt.run(&[&5], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"Something Something 5 Something"#);
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"Default case"#);
}

#[test]
fn switch_pattern() {
    let fm = func_man::default_func_man();
    let tt = TreeTemplate::from_str(
        r#"
        {{- switch $0 $1}}
        {{- case <x:?(lt @ 4)> 2 -}}
            It's {{$x}} and 2
        {{- case ?(eq @ 5) -}}            
            Something Something 5 Something
        {{- case -}}
            Default case
        {{- /switch -}}
            "#,
    )
    .unwrap();
    let res = tt.run(&[&3, &2], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"It's 3 and 2"#);
    let res = tt.run(&[&5], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"Something Something 5 Something"#);
    let res = tt.run(&[], &mut NoTemplates, &fm).unwrap();
    assert_eq!(res, r#"Default case"#);
}
