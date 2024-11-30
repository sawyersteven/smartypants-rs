# smartypants-rs
A rust port of John Gruber's Smartypants


# Usage
```
Usage: smartypants [OPTIONS] --output <OUTPUT> <INPUT>

Arguments:
  <INPUT>
          Path to input text file

Options:
  -o, --output <OUTPUT>
          Path to output text file

  -w, --quot
          Convert html &quot; to ascii "

  -q, --quotes
          Convert quotes to curly quotes ‘’ and “”

  -b, --backticks <BACKTICKS>
          Convert backtick quotes

          [default: ignore]

          Possible values:
          - ignore
          - single: Convert single backticks ` to single quotes '
          - double: Convert double backticks `` to double quotes "
          - all:    Convert both single and double backticks to quotes

  -d, --dashes <DASHES>
          Convert dashes

          [default: basic]

          Possible values:
          - ignore
          - basic:  Convert '--' to em-dash characters
          - old:    Convert '--' and '---' to en-dash and em-dash characters
          - invert: Convert '--' and '---' to em-dash and en-dash characters

  -e, --ellipses
          Convert ellipses '...' to single character '…'

  -u, --unsmart <UNSMART>
          Convert numberic html entities (eg &#8221;)

          [default: ignore]

          Possible values:
          - ignore
          - ascii:  Convert entities to ASCII (eg ")
          - utf8:   Convert entities to UTF8 (eg “)
          - named:  Convert entities to named html (eg &ldquo;)

  -h, --help
          Print help (see a summary with '-h')
```

Or import and call `smartypants(text, opts)`