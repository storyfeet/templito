use crate::*;
use boco::*;
use err_tools::*;
use std::io::Write;
use std::process::{Command, Stdio};
use tparam::*;

pub fn exec<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() == 0 {
        e_str("Must have something to exec")?;
    }
    let c = Command::new(args[0].to_string())
        .args(args[1..].into_iter().map(|v| v.to_string()))
        .output()?;

    b_ok(TData::String(String::from_utf8(c.stdout)?))
}

pub fn exec_stdin<'a>(args: &[TBoco<'a>]) -> Result<TBoco<'a>, anyhow::Error> {
    if args.len() <= 1 {
        e_str("Must have something to exec and an arg for stdin")?;
    }

    let mut c = Command::new(args[0].to_string())
        .args(args[2..].into_iter().map(|v| v.to_string()))
        .stdin(Stdio::piped())
        .spawn()?;
    match c.stdin {
        Some(ref mut i) => {
            write!(i, "{}", args[1])?;
            i.flush()?;
        }
        None => return e_str("Stdin not available"),
    }
    let op = c.wait_with_output()?;

    b_ok(TData::String(String::from_utf8(op.stdout)?))
}

pub fn env<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() == 0 {
        e_str("env_var needs <varname>")?;
    }
    std::env::var(&args[0].to_string())
        .map(|v| TBoco::Co(TData::String(v)))
        .ok()
        .e_str("No Var")
}

pub fn env_maybe<'a>(args: &[TBoco<'a>]) -> anyhow::Result<TBoco<'a>> {
    if args.len() == 0 {
        e_str("env_var needs <varname>")?;
    }
    let s = std::env::var(&args[0].to_string()).unwrap_or(String::new());
    return b_ok(TData::String(s));
}
