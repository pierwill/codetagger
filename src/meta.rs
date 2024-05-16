//! Functions for working with metadata (tags, facets, keywords) in our docs.

use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::str::FromStr;

use regex::Regex;

use crate::files::read_lines;
use crate::types::{Language, Reason};
use crate::CODE_TABS_STRINGS_2;

// Returns true if the file needs a "code example" tag, and the Reason.
pub fn check_needs_code_example_tag(path: &str, strings: Vec<String>) -> (bool, Option<Reason>) {
    if path.contains("/includes/") {
        return (false, None);
    }

    let lines = read_lines(path);

    for line in &lines {
        for item in &strings {
            if line.contains(item) {
                return (true, Some(Reason::CodeExample(item.to_string())));
            }
        }
    }

    (false, None)
}

// Returns true if the file needs a language facet, and the Reason.
pub fn check_needs_lang_metadata(path: &str) -> (bool, Option<Reason>) {
    // TODO Handle includes
    // if path.contains("/includes/") {
    //     return (false, None);
    // }

    let lines = read_lines(path);

    let mut langs_on_page: BTreeSet<Language> = BTreeSet::new();

    for line in lines.iter() {
        if line.contains(CODE_TABS_STRINGS_2) {
            let tabids = get_tabids(&lines);
            for s in tabids {
                let lang = match Language::from_str(&s) {
                    Ok(l) => l,
                    Err(_) => continue,
                };
                langs_on_page.insert(lang);
            }
        }
    }

    if langs_on_page.is_empty() {
        return (false, None);
    } else {
        return (true, Some(Reason::Languages(langs_on_page)));
    }
}

pub fn get_meta_keywords(path: &str) -> Option<String> {
    let lines = read_lines(path);
    for line in lines.iter() {
        if line.contains(":keywords: ") {
            return Some(line.to_string());
        }
    }
    None
}

pub fn get_pl_facet_values(path: &str) -> Option<BTreeSet<Language>> {
    let contents = read_to_string(path).expect("Oops opening file");

    let re =
        Regex::new(r"\.\. facet::\n(.*):name: programming_language\n.(.*):values:(.*)").unwrap();
    let r = re.find(&contents);

    if r.is_none() {
        return None;
    }

    let values_str: Vec<_> = r.unwrap().as_str().split(":values:").collect::<Vec<_>>()[1]
        .split(",")
        .map(|s| s.trim())
        .collect();

    let mut langs: BTreeSet<Language> = BTreeSet::default();

    for v in &values_str {
        let lang = match Language::from_str(&v) {
            Ok(l) => l,
            Err(_) => continue,
        };
        langs.insert(lang);
    }

    // println!("{values_str:?}");
    // println!("{langs:?}");
    Some(langs)
}

pub fn get_tabids(lines: &Vec<String>) -> Vec<String> {
    let mut tabids: Vec<String> = vec![];
    for line in lines.iter() {
        if line.contains(":tabid:") {
            tabids.push(line.split(":tabid:").map(|s| s.trim()).collect::<Vec<_>>()[1].to_string());
        }
    }
    tabids
}
