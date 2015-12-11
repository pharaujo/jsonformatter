extern crate serde_json;

use serde_json::Value;
use serde_json::error::*;
use std::io::{ErrorKind, Read, Write};

fn die<T: std::error::Error>(e: T) -> ! {
    writeln!(&mut std::io::stderr(), "error: {:?}", e).ok();
    std::process::exit(1)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut reader = match args.len() {
        1 => Box::new(std::io::stdin()) as Box<Read>,
        2 => {
            match std::fs::File::open(&args[1]) {
                Ok(f) => Box::new(f) as Box<Read>,
                Err(e) => die(e),
            }
        }
        _ => die(std::io::Error::new(ErrorKind::InvalidInput, "too many args")),
    };
    let mut string = String::new();
    reader.read_to_string(&mut string).ok().unwrap();
    let data: Value = match serde_json::de::from_str(&string) {
        Ok(v) => v,
        Err(Error::SyntaxError(ErrorCode::ExpectedSomeValue, _, _)) => {
            string = string.replace("infoCallback(", "").replace(");", "");
            match serde_json::de::from_str(&string) {
                Ok(v) => v,
                Err(e) => die(e),
            }
        }
        Err(e) => die(e),
    };
    match serde_json::ser::to_string_pretty(&data) {
        Ok(v) => print!("{}", v),
        Err(e) => die(e),
    };
}
