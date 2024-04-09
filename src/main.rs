#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod includes;

use std::fs::read_to_string;
use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};

use regex::Regex;
use walkdir::WalkDir;

const TAB_SELECTOR: &str = "tab-selector:: drivers";
const TABS_DRIVERS: &str = "tab-drivers::";

fn main() {
    let mut files_needing_tag: Vec<String> = vec![];

    for entry in WalkDir::new("/Users/wep/repos/cloud-docs/source") {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        if check_needs_tag(&filepath) {
            files_needing_tag.push(filepath.clone());
        }
    }

    println!("these files need some kind of tags: {files_needing_tag:#?}");
    for file in files_needing_tag {
        let meta_keywords = get_meta_keywords(&file);
        if meta_keywords.is_some() && meta_keywords.unwrap().contains("code example") {
            println!("{file} has code example in meta keywords");
        }
    }
}

fn check_needs_tag(path: &str) -> bool {
    let lines = read_lines(path);
    let mut needs_tag = false;

    for line in lines {
        if line.contains(TABS_DRIVERS) || line.contains(TAB_SELECTOR) {
            needs_tag = true;
        }
        for include in includes::INCLUDES.iter() {
            if line.contains(include) {
                needs_tag = true;
            }
        }
    }
    needs_tag
}

fn get_meta_keywords(path: &str) -> Option<String> {
    let lines = read_lines(path);
    for line in lines.iter() {
        if line.contains(":keywords: ") {
            return Some(line.to_string());
        }
    }
    None
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_default() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

// Add a facet directive with a programmingLanguage attribute with values that correspond to all code samples on the page.
// ```
// .. facet::
// :name: programmingLanguage
// :values: shell, csharp, javascript/typescript
// ```
fn add_facet() {}

// Each programming language used on the page
// ```
// .. meta::
// :keywords: code example, node.js
// ```
fn add_meta_keyword(path: &str) {
    let re = Regex::new(r".. meta::(.*)\n(.*):keywords:(.*)").unwrap();
    let contents = fs::read_to_string(path).expect("oops");
    let r = re.find(&contents);
    println!("{:?}", r);
}
