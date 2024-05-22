use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

// The reason a file needs tagging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    CodeExample(String),
    Languages(BTreeSet<Language>),
    NodejsTab,
    CompassTab,
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
            "c" => Ok(Self::C),
            "cpp" => Ok(Self::Cpp),
            "csharp" => Ok(Self::Csharp),
            "go" => Ok(Self::Go),
            "java-async" => Ok(Self::Java),
            "java-sync" => Ok(Self::Java),
            "javascript/typescript" => Ok(Self::Javascript),
            "kotlin-coroutine" => Ok(Self::Kotlin),
            "kotlin" => Ok(Self::Kotlin),
            "nodejs" => Ok(Self::Javascript),
            "perl" => Ok(Self::Perl),
            "php" => Ok(Self::Php),
            "python" => Ok(Self::Python),
            "ruby" => Ok(Self::Ruby),
            "rust" => Ok(Self::Rust),
            "rust-async" => Ok(Self::Rust),
            "rust-sync" => Ok(Self::Rust),
            "scala" => Ok(Self::Scala),
            "shell" => Ok(Self::Shell),
            "swift-sync" => Ok(Self::Swift),
            "swift-async" => Ok(Self::Swift),
            _ => Err(ParseLangError),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Csharp => "csharp",
            Self::Go => "go",
            Self::Java => "java",
            Self::Javascript => "javascript/typescript",
            Self::Kotlin => "kotlin",
            Self::Perl => "perl",
            Self::Php => "php",
            Self::Python => "python",
            Self::Ruby => "ruby",
            Self::Rust => "rust",
            Self::Scala => "scala",
            Self::Shell => "shell",
            Self::Swift => "swift",
        };
        write!(f, "{}", s)
    }
}
