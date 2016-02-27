extern crate tiger;
extern crate tiger_syntax as syntax;

use std::io::prelude::*;
use std::fs::{self, File};

#[test]
fn test_smoke() {
    for entry in fs::read_dir("fixtures").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().contains(".tig") {
            let path = entry.path();
            println!("Testing file: {}", path.to_str().unwrap());
            let mut file = File::open(&path).unwrap();
            let mut source = String::new();
            file.read_to_string(&mut source).expect("error reading fixture");
            tiger::compile(&source);
        }
    }
}
