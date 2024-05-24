use std::collections::HashSet;

use clap::Parser;
use itertools::Itertools;
use walkdir::WalkDir;

use codetagger::cli::Args;

const MAX_DIST: usize = 3;

fn main() {
    let args = Args::parse();
    let _debug = false;
    let _dryrun = args.dryrun;
    let repo = args.repo;

    let mut all_keywords: HashSet<String> = HashSet::default();

    for entry in WalkDir::new(&repo) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let filepath = String::from(entry_path.to_string_lossy());

        let keys = codetagger::meta::get_meta_keywords(&filepath);
        if keys.is_some() {
            for key in &keys.clone().unwrap() {
                all_keywords.insert(key.to_string());
            }
        }

        if keys.is_some() {
            for pair in keys
                .clone()
                .unwrap()
                .iter()
                .cartesian_product(keys.unwrap().iter())
            {
                let dist = edit_distance::edit_distance(pair.0, pair.1);
                if 1 <= dist && dist <= MAX_DIST {
                    println!("found similar keywords {:?}", pair);
                }
            }
        }
    }

    if args.verbose {
        println!("{:#?}", all_keywords);
    }
}
