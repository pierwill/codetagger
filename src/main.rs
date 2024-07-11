use std::collections::HashSet;

use ansi_term::Colour::White;
use clap::Parser;
use codetagger::types::JavaSyncness;
use walkdir::WalkDir;

use codetagger::cli::Args;
use codetagger::files::*;
use codetagger::includes::*;
use codetagger::meta::*;
use codetagger::types::Reason;

const CODE_TABS_STRINGS_1: &str = "tabs-selector:: drivers";
const CODE_TABS_STRINGS_2: &str = "tabs-drivers::";

/// A pair of a file path and optional Reason for needed tagging.
/// This is hashable so that we can have multiple entries per file.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct FileAndReason(String, Option<Reason>);

fn main() {
    let args = Args::parse();
    let _debug = false;
    let dryrun = args.dryrun;
    let repo = args.repo;

    let mut files_needing_tag_and_reason: HashSet<FileAndReason> = HashSet::default();

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
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_lang_metadata(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_nodejs_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_java_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_compass_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        // Atlas
        let reason = check_needs_atlas_api_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_atlas_cli_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }

        let reason = check_needs_atlas_ui_tag(&filepath);
        if reason.is_some() {
            files_needing_tag_and_reason.insert(FileAndReason(filepath.clone(), reason));
        }
    }

    if args.verbose {
        dbg!(&files_needing_tag_and_reason);
    }

    println!("📝 Tagging for programming language facets ...");
    let mut already_edited: HashSet<String> = HashSet::default();
    for FileAndReason(file, reason) in &files_needing_tag_and_reason {
        if let Some(Reason::Languages(langs)) = reason {
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
    }

    for FileAndReason(file, reason) in &files_needing_tag_and_reason {
        if reason.is_some() {
            match reason.clone().unwrap() {
                Reason::CodeExample(_) => tag_with_keyword(file, "code example", dryrun),
                Reason::NodejsTab => tag_with_keyword(file, "node.js", dryrun),
                Reason::CompassTab => tag_with_keyword(file, "compass", dryrun),
                Reason::AtlasApiTab => tag_with_keyword(file, "atlas api", dryrun),
                Reason::AtlasCliTab => tag_with_keyword(file, "atlas cli", dryrun),
                Reason::AtlasUiTab => tag_with_keyword(file, "atlas ui", dryrun),
                Reason::Java(JavaSyncness::Both) => {
                    tag_with_keyword(file, "java async", dryrun);
                    tag_with_keyword(file, "java sync", dryrun);
                }
                Reason::Java(JavaSyncness::Async) => tag_with_keyword(file, "java async", dryrun),
                Reason::Java(JavaSyncness::Sync) => tag_with_keyword(file, "java sync", dryrun),
                Reason::Languages(_) => continue,
            }
        }
    }

    if dryrun {
        println!(
            "{}",
            White.paint("\n👉 This was a dry run.\nTo update files, run with `--dryrun=false`.")
        );
    }

    // PANIC if we have two PL facets!
    for entry in WalkDir::new(&repo) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        let lines = read_lines(&filepath);
        let mut count = 0;
        for line in lines {
            if line.contains("programming_language") {
                count += 1
            }
            if count == 2 {
                panic!("too many PL lines: {filepath}")
            }
        }
    }
}

fn tag_with_keyword(file: &str, s: &str, dryrun: bool) {
    let meta_keywords: Option<Vec<String>> = get_meta_keywords(file);

    // File doesn't have any meta keywords.
    // Add them! (But skip includes.)
    if meta_keywords.is_none() && !file.contains("/includes/") {
        add_meta_keywords(file, dryrun);
    }

    if meta_keywords.is_some() && meta_keywords.unwrap().contains(&String::from(s)) {
        return;
    } else if !file.contains("/includes/") {
        add_to_meta_keywords(file, s, dryrun)
    }
}
