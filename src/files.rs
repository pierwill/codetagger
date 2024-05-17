//! Functions for working with files.

use std::collections::BTreeSet;
use std::fs::read_to_string;

use regex::Regex;

use crate::types::Language;

// This is a simple macro named `say_hello`.
macro_rules! dont_edit_includes_direct {
    ($path:expr) => {
        // The macro will expand into the contents of this block.
        assert!(
            !$path.contains("/includes/"),
            "We don't want to directly edit any files in `/includes/`.\nTried to edit {}.",
            $path
        )
    };
}

pub fn add_to_meta_keywords(path: &str, dryrun: bool) {
    dont_edit_includes_direct!(path);

    let contents = read_to_string(path).expect("oops");

    let re = Regex::new(r"(.*):keywords:(.*)").unwrap();
    let r = re.find(&contents);

    if r.is_some() {
        let rmatch = r.unwrap().as_str();
        // Need to convert `$` to ``$$`` otherwise strings like `$vectorSearch`
        // disappear when we do the replacement.
        // According to the regex crate docs, "To write a literal $ use $$"
        // (https://docs.rs/regex/1.10.4/regex/struct.Regex.html#replacement-string-syntax).
        let rmatch = rmatch.replace('$', "$$");
        let newstring = rmatch + ", code example";
        let newcontents: String = re.replace(&contents, newstring).to_string();
        if !dryrun {
            std::fs::write(path, newcontents).expect("Unable to write file");
        }
        println!("✓ File edited: {path}");
    }
}

pub fn add_meta_keywords(path: &str, dryrun: bool) {
    dont_edit_includes_direct!(path);

    let mut contents = read_to_string(path).expect("oops");
    contents.insert_str(0, ".. meta::\n   :keywords: code example\n\n");
    if !dryrun {
        std::fs::write(path, contents).expect("Unable to write file");
    }
    println!("✓ File edited: {path}");
}

pub fn add_pl_facet(path: &str, dryrun: bool, langs: BTreeSet<Language>) {
    dont_edit_includes_direct!(path);

    let mut facet = String::from(".. facet::\n   :name: programming_language\n   :values: ");
    for lang in langs {
        facet += &lang.to_string();
        facet += ", ";
    }
    facet.pop(); // remove trailing whitespace
    facet.pop(); // remove trailing comma
    facet += "\n\n";

    let mut contents = read_to_string(path).expect("oops");
    contents.insert_str(0, &facet);

    if !dryrun {
        std::fs::write(path, contents).expect("Unable to write file");
    }
    println!("✓ File edited: {path}");
}

pub fn rm_pl_facet(path: &str, dryrun: bool) {
    dont_edit_includes_direct!(path);

    let contents = read_to_string(path).expect("oops");
    let re = Regex::new(
        r"\.\. facet::(.*)\n(.*):name: programming_language(.*)\n.(.*):values:(.*)(\n*)",
    )
    .unwrap();
    let r = re.find(&contents);

    if r.is_some() {
        let newstring = "";
        let newcontents: String = re.replace(&contents, newstring).to_string();
        if !dryrun {
            std::fs::write(path, newcontents).expect("Unable to write file");
        }
    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_default() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
