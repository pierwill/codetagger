use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

// The reason a file needs tagging.
#[derive(Debug, Clone)]
pub enum Reason {
    CodeExample(String),
    Languages(BTreeSet<Language>),
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Language {
    C,
    Cpp,
    Csharp,
    Go,
    Java,
    Javascript,
    Kotlin,
    Nodejs,
    Perl,
    Php,
    Python,
    Ruby,
    Rust,
    Scala,
    Shell,
    Swift,
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
            "java-async" => Ok(Self::Java),
            "java-sync" => Ok(Language::Java),
            "javascript/typescript" => Ok(Language::Javascript),
            "kotlin-coroutine" => Ok(Language::Kotlin),
            "kotlin" => Ok(Language::Kotlin),
            "nodejs" => Ok(Language::Nodejs),
            "perl" => Ok(Language::Perl),
            "php" => Ok(Language::Php),
            "python" => Ok(Language::Python),
            "ruby" => Ok(Language::Ruby),
            "rust" => Ok(Language::Rust),
            "scala" => Ok(Language::Scala),
            "shell" => Ok(Language::Shell),
            "swift-sync" => Ok(Language::Swift),
            "swift-async" => Ok(Language::Swift),
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
            Language::Java => "java",
            Language::Javascript => "javascript",
            Language::Kotlin => "kotlin",
            Language::Nodejs => "nodejs, javascript/typescript",
            Language::Perl => "perl",
            Language::Php => "php",
            Language::Python => "python",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
            Language::Scala => "scala",
            Language::Shell => "shell",
            Language::Swift => "swift",
        };
        write!(f, "{}", s)
    }
}
