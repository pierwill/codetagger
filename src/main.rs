use std::collections::BTreeMap;
use std::fs::read_to_string;

use ansi_term::Colour::White;
use clap::ArgAction;
use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

const CODE_TABS_STRINGS_1: &str = "tabs-selector:: drivers";
const CODE_TABS_STRINGS_2: &str = "tabs-drivers::";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// In order to make changes to the files,
    /// run `with --dryrun=false`.
    #[clap(long, short,
           default_missing_value("true"), default_value("true"), num_args(0..=1),
           require_equals(true), action = ArgAction::Set)]
    dryrun: bool,
    /// Path to the root of the target repo.
    #[arg(short, long)]
    repo: String,
    /// Print information on matches.
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let _debug = false;
    let dryrun = args.dryrun;
    let repo = args.repo;

    let mut files_needing_tag_and_reason: BTreeMap<String, Option<String>> = BTreeMap::default();
    let mut stringlist: Vec<String> = vec![];
    let mut includes_with_code_tabs: Vec<String> = get_includes_with_code_tabs(repo.clone());
    stringlist.append(&mut includes_with_code_tabs);
    stringlist.push(CODE_TABS_STRINGS_1.to_string());
    stringlist.push(CODE_TABS_STRINGS_2.to_string());

    if args.verbose {
        println!("Strings to look for: {:#?}", stringlist);
    }

    // Loop through all sub directories looking
    // for files that need tagging.
    println!("👀 Looking for files that need tagging...");
    for entry in WalkDir::new(repo) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        let (needs_tag, reason) = check_needs_tag(&filepath, stringlist.clone());

        if needs_tag {
            files_needing_tag_and_reason.insert(filepath.clone(), reason);
        }
        // use indexmap?
    }

    if args.verbose {
        println!(
            "Found {} files:\n{:#?}",
            files_needing_tag_and_reason.len(),
            files_needing_tag_and_reason,
        );
    }

    // For all files needing tagging,
    // add `code example` to meta keywords
    println!("📝 Tagging...");
    for (file, _) in files_needing_tag_and_reason {
        let meta_keywords: Option<String> = get_meta_keywords(&file);
        let has_meta_keywords: bool = meta_keywords.is_some();

        if has_meta_keywords && meta_keywords.unwrap().contains("code example") {
            // File has already has `code example` in meta keywords
            continue;
        } else {
            add_to_meta_keywords(&file, dryrun)
        }

        // File doesn't have any meta keywords.
        // Add them! (But skip includes.)
        if !has_meta_keywords && !file.contains("/includes/") {
            add_meta_keywords(&file, dryrun);
        }
    }

    if dryrun {
        println!(
            "{}",
            White.paint("\n👉 This was a dry run.\nTo update files, run with `--dryrun=false`.")
        );
    }
}

// Looks through the includes/ directory to find files
// containing code tabs.
fn get_includes_with_code_tabs(repo: String) -> Vec<String> {
    let mut includes_with_code_tabs: Vec<String> = vec![];

    for entry in WalkDir::new(repo + "source/includes") {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());
        let lines = read_lines(&filepath);

        for line in lines {
            if line.contains(CODE_TABS_STRINGS_1) || line.contains(CODE_TABS_STRINGS_2) {
                includes_with_code_tabs
                    // We only want the part of the path starting with "/includes/",
                    // so split at "source".
                    .push(filepath.split("/source/").collect::<Vec<_>>()[1].to_string());
                break;
            }
        }
    }
    includes_with_code_tabs
}

// Returns true if it needs a tag, and the string value that matched.
// This string is the reason we added the tag.
fn check_needs_tag(path: &str, strings: Vec<String>) -> (bool, Option<String>) {
    if path.contains("/includes/") {
        return (false, None);
    }

    let lines = read_lines(path);

    for line in lines {
        for item in &strings {
            if line.contains(item) {
                return (true, Some(item.to_string()));
            }
        }
    }
    (false, None)
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

fn add_to_meta_keywords(path: &str, dryrun: bool) {
    let contents = read_to_string(path).expect("oops");

    let re = Regex::new(r"(.*):keywords:(.*)").unwrap();
    let r = re.find(&contents);

    if r.is_some() {
        let rmatch = r.unwrap().as_str();
        // Need to convert `$` to ``$$`` otherwise strings like `$vectorSearch`
        // disappear when we do the replacement.
        // According to the regex crate docs, "To write a literal $ use $$"
        // (https://docs.rs/regex/1.10.4/regex/struct.Regex.html#replacement-string-syntax).
        let rmatch = rmatch.replace("$", "$$");
        let newstring = rmatch + ", code example";
        let newcontents: String = re.replace(&contents, newstring).to_string();
        if !dryrun {
            std::fs::write(path, newcontents).expect("Unable to write file");
        }
        println!("✓ File edited: {path}");
    }
}

fn add_meta_keywords(path: &str, dryrun: bool) {
    let mut contents = read_to_string(path).expect("oops");
    contents.insert_str(0, ".. meta::\n   :keywords: code example\n\n");
    if !dryrun {
        std::fs::write(path, contents).expect("Unable to write file");
    }
    println!("✓ File edited: {path}");
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_default() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
