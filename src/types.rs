use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

// The reason a file needs tagging.
#[derive(Debug, Clone)]
pub enum Reason {
    CodeExample(String),
    Languages(HashSet<Language>),
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Language {
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
pub struct ParseLangError;

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
