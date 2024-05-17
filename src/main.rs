use std::collections::BTreeMap;
use std::collections::BTreeSet;

use ansi_term::Colour::White;
use clap::{ArgAction, Parser};
use walkdir::WalkDir;

mod files;
mod includes;
mod meta;
mod types;

use crate::files::*;
use crate::includes::*;
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
    for entry in WalkDir::new(&repo) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        let reason = check_needs_code_example_tag(&filepath, match_string_list.clone());
        if reason.is_some() {
            files_needing_tag_and_reason.insert(filepath.clone(), reason);
        }

        let reason = check_needs_lang_metadata(&filepath);
        if reason.is_some() {
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
    #[allow(clippy::for_kv_map)]
    for (file, _reason) in &files_needing_tag_and_reason {
        let meta_keywords: Option<String> = get_meta_keywords(file);
        let has_meta_keywords: bool = meta_keywords.is_some();

        if has_meta_keywords && meta_keywords.unwrap().contains("code example") {
            // File has already has `code example` in meta keywords
            if args.verbose {
                println!("💁 {file} already has code-example tag");
            }
            continue;
        } else if !file.contains("/includes/") {
            add_to_meta_keywords(file, dryrun)
        }

        // File doesn't have any meta keywords.
        // Add them! (But skip includes.)
        if !has_meta_keywords && !file.contains("/includes/") {
            add_meta_keywords(file, dryrun);
        }
    }

    println!("📝 Tagging for programming language facets ...");
    let mut already_edited: BTreeSet<String> = BTreeSet::default();
    for (file, reason) in &files_needing_tag_and_reason {
        let existing_facet_values: Option<_> = get_pl_facet_values(file);

        let langs = match reason.clone().unwrap() {
            Reason::CodeExample(_) => continue, // actually this case can't happen?
            Reason::Languages(l) => l,
        };

        if existing_facet_values.is_some() {
            if args.verbose {
                println!("💁 {file} already has PL facet");
            }
            if !langs.is_empty() {
                rm_pl_facet(file, dryrun);
            }
        }

        if !file.contains("/includes/") {
            add_pl_facet(file, dryrun, langs.clone());
            already_edited.insert(file.to_string());
        }

        let files_that_include_this_file =
            get_files_that_include_this_file(file.clone(), repo.clone(), args.verbose);

        for file in files_that_include_this_file {
            if !already_edited.contains(&file) && !file.contains("/includes/") {
                add_pl_facet(&file, dryrun, langs.clone());
                already_edited.insert(file.to_string());
            }
        }
    }

    if dryrun {
        println!(
            "{}",
            White.paint("\n👉 This was a dry run.\nTo update files, run with `--dryrun=false`.")
        );
    }
}
