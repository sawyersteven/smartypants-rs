use fancy_regex::Regex;

// Construct to re-compile regex. Performance gains depend on the number of
// tokens parsed from the input. A test of a large html file shows a 5-7x
// speed increase
pub struct Reg {
    pub skip: Regex,
    pub dash_basic: Regex,
    pub tokenize: Regex,
    // todo: These need better names
    pub quotes1: Regex,
    pub quotes2: Regex,
    pub quotes3: Regex,
    pub quotes4: Regex,
    pub quotes5: Regex,
    pub quotes6: Regex,
    pub quotes7: Regex,
    pub quotes8: Regex,
    pub quotes9: Regex,
    pub quotes10: Regex,
    pub quotes11: Regex,
}
impl Reg {
    const SKIP_TAGS: [&str; 8] = ["pre", "samp", "code", "tt", "kbd", "script", "style", "math"];
    const PUNCT: &str = r##"[!"#\$\%'()*+,-.\/:;<=>?\@\[\\\]\^_`{|}~]"##;
    const CLOSE_CLS: &str = r"([^ \t\r\n\[\{\(\-])";

    pub fn new() -> Self {
        return Reg {
            skip: Regex::new(&format!("<(/)?({})[^>]*>", Self::SKIP_TAGS.join("|"))).unwrap(),
            dash_basic: Regex::new("(?<!-)-{2}(?!-)").unwrap(),
            tokenize: Regex::new(r"([^<]*)(<!--.*?--\s*>|<[^>]*>)").unwrap(),
            quotes1: Regex::new(&*format!("{}{}{}", r"^'(?=", Self::PUNCT, r"\\B)")).unwrap(),
            quotes2: Regex::new(&*format!("{}{}{}", r#"^"(?="#, Self::PUNCT, r"\\B)")).unwrap(),
            quotes3: Regex::new("\"'(?=\\w)").unwrap(),
            quotes4: Regex::new("'\"(?=\\w)").unwrap(),
            quotes5: Regex::new(r"\b'(?=\d{2}s)").unwrap(),
            quotes6: Regex::new(r"(\s|&nbsp;|--|&[mn]dash;|&#8211;|&#8212;|&\#x201[34];)'(?=\w)").unwrap(),
            quotes7: Regex::new(&*format!("{}{}{}", Self::CLOSE_CLS, "'", r"(?!\s|s\b|\d)")).unwrap(),
            quotes8: Regex::new(&*format!("{}{}{}", Self::CLOSE_CLS, "'", r"(\s|s\b)")).unwrap(),
            quotes9: Regex::new(r#"(\s|&nbsp;|--|&[mn]dash;|&#8211;|&#8212;|&\#x201[34];)"(?=\w)"#).unwrap(),
            quotes10: Regex::new(&*format!("{}{}{}", Self::CLOSE_CLS, "?\"", r"(?=\s)")).unwrap(),
            quotes11: Regex::new(&*format!("{}{}", Self::CLOSE_CLS, "\"")).unwrap(),
        };
    }
}
