mod cli;
mod opts;
mod reg;

use clap::Parser;
use cli::{Cli, Unsmart};
use fancy_regex::Captures;
use opts::Opts;
use reg::Reg;
use std::collections::HashMap;

fn main() {
    let cli_args = Cli::parse();
    let mut text = match std::fs::read_to_string(&cli_args.input) {
        Ok(t) => t,
        Err(e) => {
            println!("Unable to open input file {}", e);
            return;
        }
    };

    text = smartypants(&text, &Opts::from(&cli_args));

    match std::fs::write(&cli_args.output, text) {
        Ok(_) => println!("Wrote to {}", &cli_args.output),
        Err(e) => {
            println!("Unable to write to output file {}", e);
            return;
        }
    }
}

fn smartypants(text: &str, opts: &Opts) -> String {
    let reg = Reg::new();

    let tokens = tokenize(text, &reg);
    let mut res = Vec::new();

    let mut prev_token_last_char = 0 as char;

    let mut skipped_tags = Vec::new();

    let mut in_pre = false;

    for [tag, content] in tokens {
        if tag == "tag" {
            res.push(content.to_string());

            if let Some(caps) = reg.skip.captures(content).ok().and_then(|x| x) {
                let cap2 = caps.get(2).unwrap().as_str().to_lowercase();
                if caps.get(1).is_none() {
                    skipped_tags.push(cap2);
                    in_pre = true;
                } else {
                    if skipped_tags.len() > 0 {
                        let tag = skipped_tags.last().unwrap();
                        if cap2 == *tag {
                            skipped_tags.pop();
                        }
                    }
                    if skipped_tags.len() == 0 {
                        in_pre = false;
                    }
                }
            }
        } else {
            let mut t = content.to_string();
            let last_char = t.chars().last().unwrap();
            if !in_pre {
                t = unescape_text(content);

                if opts.quot {
                    t = t.replace("&quot;", "\"");
                }

                match opts.dashes {
                    cli::Dash::Ignore => {}
                    cli::Dash::Basic => {
                        t = reg.dash_basic.replace_all(&t, "&#8212;").to_string();
                    }
                    cli::Dash::Old => {
                        t = t.replace("---", "&#8212;").replace("--", "&#8211;");
                    }
                    cli::Dash::Invert => {
                        t = t.replace("---", "&#8211;").replace("--", "&#8212;");
                    }
                }

                if opts.ellipses {
                    t = t.replace("...", "&#8230;");
                    t = t.replace(". . .", "&#8230;");
                }

                match opts.backticks {
                    cli::Backtick::Ignore => {}
                    cli::Backtick::All => {
                        t = t.replace("``", "&#8220;");
                        t = t.replace("''", "&#8221;");

                        t = t.replace("`", "&#8216;");
                        t = t.replace("'", "&#8217;");
                    }
                    cli::Backtick::Single => {
                        t = t.replace("`", "&#8216;");
                        t = t.replace("'", "&#8217;");
                    }
                    cli::Backtick::Double => {
                        t = t.replace("``", "&#8220;");
                        t = t.replace("''", "&#8221;");
                    }
                }

                if opts.quotes {
                    // Case or single ' string
                    if t == "'" {
                        if !prev_token_last_char.is_whitespace() {
                            t = "&#8217;".to_string();
                        } else {
                            t = "&#8216;".to_string();
                        }
                    } else if t == "\"" {
                        if !prev_token_last_char.is_whitespace() {
                            t = "&#8221;".to_string();
                        } else {
                            t = "&#8220;".to_string();
                        }
                    } else {
                        t = convert_quotes(&t, &reg);
                    }
                }

                if opts.unsmart != Unsmart::Ignore {
                    t = unsmart(&t, &opts.unsmart);
                }
            }

            prev_token_last_char = last_char;
            res.push(t.to_string())
        }
    }
    return res.join("");
}

fn tokenize<'a>(text: &'a str, reg: &'a Reg) -> Vec<[&'a str; 2]> {
    let mut tokens = Vec::new();

    let mut offset = 0;
    while let Some(caps) = reg.tokenize.captures_from_pos(text, offset).ok().and_then(|x| x) {
        caps.get(1).and_then(|x| {
            let x = x.as_str();
            if x.len() > 0 {
                return Some(tokens.push(["text", x]));
            }
            return Some(());
        });

        let tag = caps.get(2).unwrap().as_str();
        let mut t = "tag";

        if tag.starts_with("<!--") {
            if tag[4..]
                .trim_end_matches('>')
                .trim_end()
                .trim_end_matches('-')
                .contains("--")
            {
                t = "text";
            }
        }
        tokens.push([t, tag]);
        offset = match caps.get(caps.len() - 1) {
            Some(c) => c.end(),
            None => panic!(),
        };
    }
    if offset < text.len() {
        tokens.push(["text", &text[offset..]]);
    }
    return tokens;
}

// Replace escaped characters with the http &code equivalent to allow forcing
// non-smart punctuation characters
fn unescape_text(text: &str) -> String {
    const ESC_PAIRS: [(&str, &str); 6] = [
        (r"\\", "&#92;"),  // \
        ("\\\"", "&#34;"), // "
        ("\\'", "&#39;"),  // '
        ("\\.", "&#46;"),  // .
        ("\\-", "&#45;"),  // -
        ("\\`", "&#96;"),  // ``
    ];
    let mut text = text.to_owned();
    for (i, o) in ESC_PAIRS {
        text = text.replace(i, o);
    }
    return text;
}

// Convert quotes in text (eg not tags or comments) into numeric HTML
// curly quote entities
fn convert_quotes(text: &str, reg: &Reg) -> String {
    let mut text = text.to_string();

    // If the first char is a " followed by puncutation that isn't a word break
    // force-close the quote
    text = reg.quotes1.replace_all(&text, "&#8217;").to_string();
    text = reg.quotes2.replace_all(&text, "&#8221;").to_string();

    // Handle double quotes eg <span>Fry said "'It was Fry!' is what Zoidberg told me"
    text = reg.quotes3.replace_all(&text, "&#8220;&#8216;").to_string();
    text = reg.quotes4.replace_all(&text, "&#8216;&#8220;").to_string();

    // Handle year abbreviations eg '90s or '40s
    text = reg.quotes5.replace_all(&text, "&#8217;").to_string();

    // Handle opening single-quotes
    text = reg
        .quotes6
        .replace(&text, |caps: &Captures| format!("{}&#8216;", &caps[1]))
        .to_string();

    // Handle closing single-quotes
    text = reg
        .quotes7
        .replace(&text, |caps: &Captures| format!("{}&#8217;", &caps[1]))
        .to_string();

    text = reg
        .quotes8
        .replace(&text, |caps: &Captures| format!("{}&#8217;{}", &caps[1], &caps[2]))
        .to_string();

    // Any remaining get swapped for open-single quote
    text = text.replace("'", "&#8216;");

    // Handle opening double-quotes
    text = reg
        .quotes9
        .replace(&text, |caps: &Captures| format!("{}&#8220;", &caps[1]))
        .to_string();

    // Handle closing double-quotes
    text = reg
        .quotes10
        .replace(&text, |caps: &Captures| format!("{}&#8221;", &caps[1]))
        .to_string();

    text = reg
        .quotes11
        .replace(&text, |caps: &Captures| format!("{}&#8221;", &caps[1]))
        .to_string();

    // Any remaining get swapped for open-double quote
    text = text.replace('"', "&#8220;");

    return text;
}

// Convert numeric html entities eg &#3211; into other representations of
// those characters
fn unsmart(text: &str, mode: &Unsmart) -> String {
    let mut text = text.to_string();
    let i = match mode {
        Unsmart::Ignore => return text, // shouldn't be able to reach this
        Unsmart::ASCII => 0,
        Unsmart::UTF8 => 1,
        Unsmart::Named => 2,
    };

    let repl_table = HashMap::from([
        ("&#8211;", ["-", "–", "&ndash;"]),
        ("&#8212;", ["--", "—", "&mdash;"]),
        ("&#8216;", ["'", "‘", "&lsquo;"]),
        ("&#8217;", ["'", "’", "&rsquo;"]),
        ("&#8220;", ["\"", "“", "&ldquo;"]),
        ("&#8221;", ["\"", "”", "&rdquo;"]),
        ("&#8230;", ["...", "…", "&hellip;"]),
    ]);

    for (k, v) in repl_table {
        text = text.replace(k, v[i]);
    }

    return text;
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use cli::{Backtick, Dash};

    #[test]
    fn test_dates() {
        let opts = make_default_opts();

        assert_eq!(smartypants("1230-40's", &opts), "1230-40&#8217;s");
        assert_eq!(smartypants("1230-'40s", &opts), "1230-&#8216;40s");
        assert_eq!(smartypants("1230--'40s", &opts), "1230&#8212;&#8216;40s");
        assert_eq!(smartypants("1990s", &opts), "1990s");
        assert_eq!(smartypants("1990's", &opts), "1990&#8217;s");
        assert_eq!(smartypants("foo bar '40s", &opts), "foo bar &#8216;40s");
        assert_eq!(smartypants("'90s", &opts), "&#8216;90s");
    }

    #[test]
    // test that everything inside a skipped tag remains untouched
    fn test_skip_tags() {
        let opts = Opts {
            quotes: true,
            backticks: Backtick::All,
            dashes: cli::Dash::Old,
            ellipses: true,
            quot: true,
            unsmart: Unsmart::Ignore,
        };

        const TEXT: &str = r#"<script type="text/javascript">
            document.write(' `${window.location}` <a href="' + "http://www.duckduckgo.com" + '">' + "smartypants" + "</a>");
            </script>"#;

        let t = smartypants(TEXT, &opts);
        assert_eq!(t, TEXT);

        const TEXT2: &str = r#"<pre>"''"<a>'""`'"<b>'""'...'"`"''<////>'''```""'"""'""</pre>"#;
        let t = smartypants(TEXT2, &opts);
        assert_eq!(t, TEXT2);

        const TEXT3: &str = r#"<pre>"""""</pre><div>"hi"</div><pre>"""""</pre>"#;
        const WANT3: &str = r#"<pre>"""""</pre><div>&#8220;hi&#8221;</div><pre>"""""</pre>"#;
        let t = smartypants(TEXT3, &opts);
        assert_eq!(t, WANT3);
    }

    #[test]
    fn test_quot() {
        let mut opts = make_default_opts();

        let text = "<p>The Professor said &quot;Let's see...&quot; This is the problem <code>if true{\n\t //&quot;Okay&quot;</code></p>";

        let t = smartypants(text, &opts);
        let e = "<p>The Professor said &quot;Let&#8217;s see...&quot; This is the problem <code>if true{\n\t //&quot;Okay&quot;</code></p>";
        assert_eq!(t, e);

        // convert html &quot; to ascii "
        opts.quot = true;
        opts.quotes = false;
        let t = smartypants(text, &opts);
        let e = "<p>The Professor said \"Let\'s see...\" This is the problem <code>if true{\n\t //&quot;Okay&quot;</code></p>";
        assert_eq!(t, e);

        // convert &quot; to ", then " to fancy quotes
        opts.quotes = true;
        let t = smartypants(text, &opts);
        let e = "<p>The Professor said &#8220;Let&#8217;s see...&#8221; This is the problem <code>if true{\n\t //&quot;Okay&quot;</code></p>";
        assert_eq!(t, e);
    }

    #[test]
    fn test_backticks() {
        let mut opts = make_default_opts();
        opts.backticks = Backtick::Double;
        let t = smartypants("``bar`` but not `bar`", &opts);
        assert_eq!(t, r#"&#8220;bar&#8220; but not `bar`"#);

        opts.backticks = Backtick::Single;
        let t = smartypants("`bar` and dbl ``bar``", &opts);
        assert_eq!(t, r#"&#8216;bar&#8216; and dbl &#8216;&#8216;bar&#8216;&#8216;"#);

        opts.backticks = Backtick::All;
        let t = smartypants("`bar` and ``bar``", &opts);
        assert_eq!(t, r#"&#8216;bar&#8216; and &#8220;bar&#8220;"#);
    }

    #[test]
    fn test_dashes() {
        let mut opts = make_default_opts();

        opts.dashes = Dash::Basic;
        let t = smartypants("This is em-dash -- but not this ---", &opts);
        assert_eq!(t, "This is em-dash &#8212; but not this ---");

        opts.dashes = Dash::Old;
        let t = smartypants("This is en-dash -- this is em-dash ---", &opts);
        assert_eq!(t, "This is en-dash &#8211; this is em-dash &#8212;");

        opts.dashes = Dash::Invert;
        let t = smartypants("This is en-dash -- this is em-dash ---", &opts);
        assert_eq!(t, "This is en-dash &#8212; this is em-dash &#8211;");
    }

    #[test]
    fn test_comments() {
        let opts = make_default_opts();

        assert_eq!(smartypants("--", &opts), "&#8212;");
        assert_eq!(smartypants("-->", &opts), "&#8212;>");
        assert_eq!(smartypants("-- \t  >", &opts), "&#8212; \t  >");

        // convert -- outside comment
        const TEXT: &str = r#"<!-- "bing" --> beep--boop <!-- "bang" -->"#;
        let t = smartypants(TEXT, &opts);
        const WANT: &str = r#"<!-- "bing" --> beep&#8212;boop <!-- "bang" -->"#;
        assert_eq!(t, WANT);

        // convert " outside comment
        const TEXT2: &str = r#"<p>foo -- "bar"<!-- baz-qux\n<p>beep "boop"</p>\n-->\n</p>"#;
        let t = smartypants(TEXT2, &opts);
        const WANT2: &str = r#"<p>foo &#8212; &#8220;bar&#8221;<!-- baz-qux\n<p>beep "boop"</p>\n-->\n</p>"#;
        assert_eq!(t, WANT2);

        // nothing should be converted inside comments
        for text in [
            "<!-- this is a comment -->",
            "<!-- <li>foo-bar-baz-qux</li> -->",
            r#"<!-- "beep" --> <!-- "boop" -->"#,
        ] {
            assert_eq!(smartypants(text, &opts), text)
        }

        // not valid comments, so convert
        assert_eq!(
            smartypants("<!-- -- -- -->", &opts),
            "<!&#8212; &#8212; &#8212; &#8212;>"
        );
        assert_eq!(smartypants("<!-- -- -- \t >", &opts), "<!&#8212; &#8212; &#8212; \t >");
    }

    #[test]
    fn test_unsmart_entities() {
        let mut opts = make_default_opts();
        opts.unsmart = Unsmart::UTF8;
        assert_eq!(smartypants(r#""quote here""#, &opts), "“quote here”");

        opts.unsmart = Unsmart::Named;
        assert_eq!(smartypants(r#""quote here""#, &opts), "&ldquo;quote here&rdquo;");

        opts.unsmart = Unsmart::ASCII;
        assert_eq!(smartypants(r#""quote here""#, &opts), r#""quote here""#);
    }

    fn make_default_opts() -> Opts {
        return Opts::from(&Cli::parse_from(["bin", "inp", "-o", "outp"].iter()));
    }
}
