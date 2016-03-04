extern crate tiger_syntax as syntax;
#[macro_use]
extern crate tiger;

use std::env;
use std::io::prelude::*;
use std::fs::File;

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

                tiger::driver::compile(&source);
            },
            Err(e) => {
                error!("no such file: {} <{}>", "driver", source_file_path, e);
            },
        };
    } else {
        error!("no file given.", "driver");
    }
}
