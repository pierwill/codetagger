#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use walkdir::WalkDir;

const CODE_BLOCK: &str = ".. code-block::";
const TAG: &str = ".. meta:: \n:keywords: code-example";

fn main() {
    let mut files_with_code_examples: Vec<_> = vec![];

    for entry in WalkDir::new("../source/") {
        let entry = entry.unwrap();
        let entrypath = entry.clone().path();

        if let Ok(lines) = std::fs::read_to_string(entrypath) {
            for line in lines.lines() {
                if line.contains(CODE_BLOCK) {
                    files_with_code_examples.push(entrypath.display());
                    // println!("{}", entrypath.display());
                    println!("yes");
                    break;
                }

                // insert the tag
            }
        }
    }
    println!("{:?}", files_with_code_examples);
}
