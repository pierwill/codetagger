use std::fs::read_to_string;

use ansi_term::Colour::White;
use clap::ArgAction;
use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

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
    /// Path to a file with list of strings to search for.
    #[arg(short, long)]
    stringsfile: String,
}

fn main() {
    let args = Args::parse();
    let debug = false;
    let dryrun = args.dryrun;
    let repo = args.repo;
    let stringslist = read_lines(&args.stringsfile);
    let mut files_needing_tag: Vec<String> = vec![];

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

        if check_needs_tag(&filepath, stringslist.clone()) {
            files_needing_tag.push(filepath.clone());
        }
        files_needing_tag.sort();
    }

    if debug {
        println!("{:#?}", files_needing_tag);
    }

    // For all files needing tagging,
    // add `code example` to meta keywords
    println!("📝 Tagging...");
    for file in files_needing_tag {
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

fn check_needs_tag(path: &str, strings: Vec<String>) -> bool {
    let lines = read_lines(path);

    for line in lines {
        for item in &strings {
            if line.contains(item) {
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
