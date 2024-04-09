#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod includes;

use std::fs::read_to_string;

use walkdir::WalkDir;

const TAB_SELECTOR: &str = "tab-selector:: drivers";
const TABS_DRIVERS: &str = "tab-drivers::";

fn main() {
    for entry in WalkDir::new("/Users/wep/repos/cloud-docs/source") {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        let needs_tag = check_needs_tag(&filepath);
        if needs_tag {
            println!("{filepath} needs tag");
        }

        let meta_keywords = get_meta_keywords(&filepath);
        if meta_keywords.is_some() && meta_keywords.unwrap().contains("code example") {
            println!("{filepath} has code example in meta keywords");
        }
    }
}

fn check_needs_tag(path: &str) -> bool {
    if contains_directive(path) || contains_code_include(path) {
        return true;
    }
    false
}

fn contains_directive(path: &str) -> bool {
    let lines = read_lines(path);
    for line in lines {
        if line.contains(TABS_DRIVERS) || line.contains(TAB_SELECTOR) {
            return true;
        }
    }
    false
}

fn contains_code_include(path: &str) -> bool {
    let lines = read_lines(path);
    for line in lines.iter() {
        for include in includes::INCLUDES.iter() {
            if line.contains(include) {
                return true;
            }
        }
    }
    false
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
