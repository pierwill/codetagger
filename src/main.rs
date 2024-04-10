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
    #[arg(short, long)]
    repo: String,
    #[arg(short, long)]
    includesfile: String,
}

const TAB_SELECTOR: &str = "tabs-selector:: drivers";
const TABS_DRIVERS: &str = "tabs-drivers::";

fn main() {
    let args = Args::parse();
    let dryrun = args.dryrun;
    let repo = args.repo;
    let includesfile = args.includesfile;
    let mut files_needing_tag: Vec<String> = vec![];

    // Loop through all sub directories looking
    // for files that need tagging.
    for entry in WalkDir::new(repo) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        if check_needs_tag(&filepath, &includesfile) {
            files_needing_tag.push(filepath.clone());
        }
    }

    // For all files needing tagging,
    // add `code example` to meta keywords
    for file in files_needing_tag {
        let meta_keywords = get_meta_keywords(&file);
        if meta_keywords.is_some() && meta_keywords.unwrap().contains("code example") {
            // File has already has `code example` in meta keywords
            continue;
        } else {
            add_meta_keyword(&file, dryrun)
        }
    }

    if dryrun {
        println!(
            "{}",
            White.paint("\nThis was a dry run.\nTo update files, run with `--dryrun=false`.")
        );
    }
}

fn check_needs_tag(path: &str, inc: &str) -> bool {
    let lines = read_lines(path);
    let includeslist = read_lines(inc);
    let mut needs_tag = false;

    for line in lines {
        if line.contains(TABS_DRIVERS) || line.contains(TAB_SELECTOR) {
            needs_tag = true;
        }
        for include in &includeslist {
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

fn add_meta_keyword(path: &str, dryrun: bool) {
    let contents = read_to_string(path).expect("oops");

    let re = Regex::new(r"(.*):keywords:(.*)").unwrap();
    let r = re.find(&contents);

    if r.is_some() {
        let rmatch = r.unwrap().as_str();
        let newstring = String::from(format!("{}{}", rmatch, ", code example"));
        let newcontents: String = re.replace(&contents, newstring).to_string();
        if !dryrun {
            std::fs::write(path, newcontents).expect("Unable to write file");
        }
        println!("File edited: {path}");
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_default() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
