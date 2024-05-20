//! Gathering information about includes files

use std::collections::BTreeSet;

use walkdir::WalkDir;

use crate::files::read_lines;
use crate::CODE_TABS_STRINGS_1;
use crate::CODE_TABS_STRINGS_2;

#[allow(unused_must_use)]
pub fn get_files_that_include_this_file(
    path: String,
    repo: String,
    verbose: bool,
) -> BTreeSet<String> {
    let mut files_that_include_this_file: BTreeSet<String> = BTreeSet::default();
    let include_path = rel_path(path);

    for entry in WalkDir::new(repo.clone()) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath_being_examined = String::from(entry_path.to_string_lossy());
        if !filepath_being_examined.contains("/source/") {
            continue;
        }

        let lines = read_lines(&filepath_being_examined);
        for line in lines {
            if line.contains(&include_path) {
                files_that_include_this_file.insert(filepath_being_examined.clone());
            }
        }
    }

    if verbose && !files_that_include_this_file.is_empty() {
        println!(
            "file {} is included by {:#?}",
            include_path, files_that_include_this_file
        );
    }

    // TODO we're not really handling nested includes yet...
    // for f in &files_that_include_this_file {
    //     if f.contains("/includes/") {
    //         println!(
    //             "Therefore, {} is included in an include! we will now do recursion",
    //             rel_path(f.to_string())
    //         );

    //         return get_files_that_include_this_file(f.to_string(), repo.clone(), verbose);
    //     }
    // }

    files_that_include_this_file
}

// Looks through the includes/ directory to find files
// containing code tabs.
pub fn get_includes_with_code_tabs(repo: String) -> Vec<String> {
    let mut includes_with_code_tabs: Vec<String> = vec![];

    for entry in WalkDir::new(repo + "source/includes") {
        let entry = entry.expect("Oops. Problem opening repo. Did you forget a trailing slash?");
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
                    .push(rel_path(filepath));
                break;
            }
        }
    }
    includes_with_code_tabs
}

fn rel_path(path: String) -> String {
    path.split("/source/").collect::<Vec<_>>()[1].to_string()
}
