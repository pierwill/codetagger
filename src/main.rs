use std::collections::BTreeMap;

use ansi_term::Colour::White;
use clap::{ArgAction, Parser};
use walkdir::WalkDir;

mod files;
mod meta;
mod types;

use crate::files::*;
use crate::meta::*;
use crate::types::Reason;

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

    let mut files_needing_tag_and_reason: BTreeMap<String, Option<Reason>> = BTreeMap::default();
    let mut match_string_list: Vec<String> = vec![];
    let mut includes_with_code_tabs: Vec<String> = get_includes_with_code_tabs(repo.clone());
    match_string_list.append(&mut includes_with_code_tabs);
    match_string_list.push(CODE_TABS_STRINGS_1.to_string());
    match_string_list.push(CODE_TABS_STRINGS_2.to_string());

    if args.verbose {
        println!("Strings to look for: {:#?}", match_string_list);
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

        let (needs_tag, reason) =
            check_needs_code_example_tag(&filepath, match_string_list.clone());
        if needs_tag {
            files_needing_tag_and_reason.insert(filepath.clone(), reason);
        }

        let (needs_tag, reason) = check_needs_lang_metadata(&filepath);
        if needs_tag {
            files_needing_tag_and_reason.insert(filepath.clone(), reason);
        }
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
    println!("📝 Tagging for \"code example\" ...");
    for (file, reason) in &files_needing_tag_and_reason {
        match reason {
            Some(Reason::CodeExample(_)) => (),
            _ => continue,
        }

        let meta_keywords: Option<String> = get_meta_keywords(&file);
        let has_meta_keywords: bool = meta_keywords.is_some();

        if has_meta_keywords && meta_keywords.unwrap().contains("code example") {
            // File has already has `code example` in meta keywords
            if args.verbose {
                println!("💁 {file} already has code-example tag");
            }
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

    println!("📝 Tagging for programming language facets ...");
    for (file, reason) in &files_needing_tag_and_reason {
        match reason {
            Some(Reason::Languages(_)) => (),
            _ => continue,
        }

        let existing_facet_values: Option<_> = get_pl_facet_values(&file);

        // TODO logic for adding facet
        // For now, skip the case where there's already a facet
        if existing_facet_values.is_some() {
            continue;
        }

        let langs = match reason.clone().unwrap() {
            Reason::CodeExample(_) => continue, // actually this case can't happen?
            Reason::Languages(l) => l,
        };
        add_pl_facet(file, dryrun, langs);
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
        let entry = entry.expect("Oops. Problem openning repo. Did you forget a trailing slash?");
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
