// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

mod includes;

use std::fs::read_to_string;

const TAB_SELECTOR: &str = "tab-selector:: drivers";
const TABS_DRIVERS: &str = "tab-drivers::";

fn main() {
    let filepath = "/Users/wep/repos/cloud-docs/source/troubleshoot-connection.txt";
    let needs_tag = check_needs_tag(filepath);

    println!("{filepath}, {needs_tag:?}");
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

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
