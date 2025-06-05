use regex::Regex;
use std::iter::Iterator;
use std::sync::LazyLock;

/// A set of inflection rules patterned after Rails' ActiveSupport::Inflections,
/// expressed as an array of tuples of (<regular expression>, <replacement>)
static PLURALISATION_RULES: LazyLock<Vec<(Regex, String)>> = LazyLock::new(|| {
    [("s$", "s"), ("$", "s")]
        .iter()
        .map(|(rule, replacement)| {
            let rule = *rule;
            (
                Regex::new(&format!("(.+){rule}")).unwrap(),
                format!("${{1}}{replacement}"),
            )
        })
        .collect()
});

static SINGULARISATION_RULES: LazyLock<Vec<(Regex, String)>> = LazyLock::new(|| {
    [("s$", "")].iter().map(|(rule, replacement)| {
        let rule = *rule;
        (
            Regex::new(&format!("(.+){rule}")).unwrap(),
            format!("${{1}}{replacement}"),
        )
    }).collect()
});

pub fn plural<S>(s: S) -> String
where
    S: Into<String>,
{
    let s: String = s.into();
    for (rule, replacement) in PLURALISATION_RULES.iter() {
        if rule.is_match(&s) {
            let captures = rule.captures_iter(&s);
            let mut plural = String::new();
            for capture in captures {
                println!("{}", replacement);
                println!("{:?}", capture);
                capture.expand(replacement, &mut plural);
            }
            println!("plural: {}", plural);
            return plural;
        }
    }
    s
}

pub fn singular<S>(s: S) -> String
where
    S: Into<String>
{
    let s: String = s.into();
    for (rule, replacement) in SINGULARISATION_RULES.iter() {
        if rule.is_match(&s) {
            let captures = rule.captures_iter(&s);
            let mut singular = String::new();
            for capture in captures {
                println!("{}", replacement);
                println!("{:?}", capture);
                capture.expand(replacement, &mut singular);
            }
            println!("singular: {}", singular);
            return singular;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(plural("author"), "authors");
        assert_eq!(plural("book"), "books");
    }

    #[test]
    fn test_singular() {
        assert_eq!(singular("authors"), "author");
        assert_eq!(singular("books"), "book");
    }
}
