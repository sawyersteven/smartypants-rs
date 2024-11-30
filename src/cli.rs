use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Path to input text file
    pub input: String,

    /// Path to output text file
    #[arg(short, long)]
    pub output: String,

    /// Convert html &quot; to ascii "
    #[arg(short('w'), long, default_value_t = false)]
    pub quot: bool,

    /// Convert quotes to curly quotes ‘’ and “”
    #[arg(short, long, default_value_t = true)]
    pub quotes: bool,

    /// Convert backtick quotes
    #[arg(short, long, default_value_t = Backtick::Ignore, value_enum)]
    pub backticks: Backtick,

    /// Convert dashes
    #[arg(short, long, default_value_t = Dash::Basic, value_enum)]
    pub dashes: Dash,

    /// Convert ellipses '...' to single character '…'
    #[arg(short, long, default_value_t = false)]
    pub ellipses: bool,

    /// Convert numberic html entities (eg &#8221;)
    #[arg(short, long, default_value_t = Unsmart::Ignore, value_enum)]
    pub unsmart: Unsmart,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Dash {
    Ignore,
    /// Convert '--' to em-dash characters
    Basic,
    /// Convert '--' and '---' to en-dash and em-dash characters
    Old,
    /// Convert '--' and '---' to em-dash and en-dash characters
    Invert,
}

impl fmt::Display for Dash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Backtick {
    Ignore,
    /// Convert single backticks ` to single quotes '
    Single,
    /// Convert double backticks `` to double quotes "
    Double,
    /// Convert both single and double backticks to quotes
    All,
}

impl fmt::Display for Backtick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Unsmart {
    Ignore,
    /// Convert entities to ASCII (eg ")
    ASCII,
    /// Convert entities to UTF8 (eg “)
    UTF8,
    /// Convert entities to named html (eg &ldquo;)
    Named,
}

impl fmt::Display for Unsmart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
