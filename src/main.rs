use std::collections::{BTreeMap, HashSet};
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

use ansi_term::Colour::White;
use clap::{ArgAction, Parser};
use regex::Regex;
use walkdir::WalkDir;

const CODE_TABS_STRINGS_1: &str = "tabs-selector:: drivers";
const CODE_TABS_STRINGS_2: &str = "tabs-drivers::";

// The reason a file needs tagging.
#[derive(Debug, Clone)]
enum Reason {
    CodeExample(String),
    Languages(HashSet<Language>),
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
enum Language {
    C,
    Cpp,
    Csharp,
    Go,
    JavaAsync,
    JavaSync,
    Javascript,
    Kotlin,
    Nodejs,
    Php,
    Python,
    Ruby,
    Rust,
    Scala,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLangError;

impl FromStr for Language {
    type Err = ParseLangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::Cpp),
            "csharp" => Ok(Language::Csharp),
            "go" => Ok(Language::Go),
            "java-async" => Ok(Self::JavaAsync),
            "java-sync" => Ok(Language::JavaSync),
            "javascript/typescript" => Ok(Language::Javascript),
            "kotlin" => Ok(Language::Kotlin),
            "nodejs" => Ok(Language::Nodejs),
            "php" => Ok(Language::Php),
            "python" => Ok(Language::Python),
            "ruby" => Ok(Language::Ruby),
            "rust" => Ok(Language::Rust),
            "scala" => Ok(Language::Scala),
            _ => Err(ParseLangError),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Csharp => "csharp",
            Language::Go => "go",
            Language::JavaAsync => "java-async",
            Language::JavaSync => "java-sync",
            Language::Javascript => "javascript",
            Language::Kotlin => "kotlin",
            Language::Nodejs => "nodejs",
            Language::Php => "php",
            Language::Python => "python",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::Scala => "scala",
        };
        write!(f, "{}", s)
    }
}

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

// Returns true if the file needs a "code example" tag, and the Reason.
fn check_needs_code_example_tag(path: &str, strings: Vec<String>) -> (bool, Option<Reason>) {
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
fn check_needs_lang_metadata(path: &str) -> (bool, Option<Reason>) {
    let lines = read_lines(path);

    let mut langs_on_page: HashSet<Language> = HashSet::new();

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

fn get_meta_keywords(path: &str) -> Option<String> {
    let lines = read_lines(path);
    for line in lines.iter() {
        if line.contains(":keywords: ") {
            return Some(line.to_string());
        }
    }
    None
}

fn get_pl_facet_values(path: &str) -> Option<HashSet<Language>> {
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

    let mut langs: HashSet<Language> = HashSet::default();

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

fn get_tabids(lines: &Vec<String>) -> Vec<String> {
    let mut tabids: Vec<String> = vec![];
    for line in lines.iter() {
        if line.contains(":tabid:") {
            tabids.push(line.split(":tabid:").map(|s| s.trim()).collect::<Vec<_>>()[1].to_string());
        }
    }
    tabids
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

fn add_pl_facet(path: &str, dryrun: bool, langs: HashSet<Language>) {
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

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap_or_default() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
