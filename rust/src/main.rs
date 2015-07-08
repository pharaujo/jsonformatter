extern crate serde;

use serde::json::{self, Value};
use serde::json::error::*;
use std::io::{Read, Write};

fn die<T: std::error::Error>(e: T) -> ! {
    writeln!(&mut std::io::stderr(), "error: {:?}", e).ok();
    std::process::exit(1)
}

fn main() {
    let mut reader = std::io::stdin();
    let mut string = String::new();
    reader.read_to_string(&mut string).ok().unwrap();
    if string.len() == 0 {
        println!("Got nothing from stdin");
        return;
    }
    let data: Value = match json::from_str(&string) {
        Ok(v) => v,
        Err(Error::SyntaxError(ErrorCode::ExpectedSomeValue, _, _)) => {
            string = string.replace("infoCallback(", "").replace(");", "");
            match json::from_str(&string) {
                Ok(v) => v,
                Err(e) => die(e)
            }
        },
        Err(e) => die(e)
    };
    match json::ser::to_string_pretty(&data) {
        Ok(v) => print!("{}", v),
        Err(e) => die(e)
    };
}

