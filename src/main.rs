extern crate tiger_syntax as syntax;
extern crate tiger;

use std::env;
use std::io::prelude::*;
use std::fs::File;

macro_rules! error {
    ($fmt:expr, $kind:expr) => {{
        print!("{} error: ", $kind);
        print!(concat!($fmt, "\n"));
        ::std::process::exit(1);
    }};
    ($fmt:expr, $kind:expr, $($arg:tt)*) => {{
        print!("{} error: ", $kind);
        print!(concat!($fmt, "\n"), $($arg)*);
        ::std::process::exit(1);
    }};
}

// TODO: Error types and `unwrap_or_error()` function on Results.

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    if let Some(source_file_path) = args.get(1) {
        match File::open(source_file_path) {
            Ok(mut source_file) => {
                let mut source = String::new();
                source_file.read_to_string(&mut source).unwrap_or_else(|e| {
                    error!("{}", "driver", e);
                });
                println!("SOURCE:");
                println!("{}", source);

                let ast = match syntax::parse(&source) {
                    Ok(a) => a,
                    Err(e) => {
                        error!("{:?}", "syntax", e);
                    }
                };
                println!("AST:");
                println!("{:#?}", ast);

                let translation = match tiger::translate(&ast) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("{:?}", "translation", e);
                    },
                };
                println!("TRANSLATION");
                println!("{:?}", translation);
            },
            Err(e) => {
                error!("no such file: {} <{}>", "driver", source_file_path, e);
            },
        };
    } else {
        error!("no file given.", "driver");
    }
}
