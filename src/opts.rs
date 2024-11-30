use crate::cli::{Backtick, Cli, Dash, Unsmart};

/// See Cli struct for description
#[derive(Debug)]
pub struct Opts {
    pub quotes: bool,
    pub backticks: Backtick,
    pub dashes: Dash,
    pub ellipses: bool,
    pub quot: bool,
    pub unsmart: Unsmart,
}

impl From<&Cli> for Opts {
    fn from(c: &Cli) -> Self {
        return Self {
            quotes: c.quotes,
            backticks: c.backticks.clone(),
            dashes: c.dashes.clone(),
            ellipses: c.ellipses,
            quot: c.quot,
            unsmart: c.unsmart.clone(),
        };
    }
}
