use crate::*;
use err::*;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn exec(args: &[TData]) -> Result<TData, anyhow::Error> {
    if args.len() == 0 {
        Err(ea_str("Must have something to exec"))?;
    }
    let c = Command::new(args[0].to_string())
        .args(args[1..].into_iter().map(|v| v.to_string()))
        .output()?;

    Ok(TData::String(String::from_utf8(c.stdout)?))
}

pub fn exec_stdin(args: &[TData]) -> Result<TData, anyhow::Error> {
    if args.len() <= 1 {
        Err(ea_str("Must have something to exec and an arg for stdin"))?;
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
        None => return Err(ea_str("Stdin not available")),
    }
    let op = c.wait_with_output()?;

    Ok(TData::String(String::from_utf8(op.stdout)?))
}
