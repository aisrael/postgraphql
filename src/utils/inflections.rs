use regex::Regex;
use std::iter::Iterator;
use std::sync::LazyLock;

/// A set of inflection rules patterned after Rails' ActiveSupport::Inflections,
/// expressed as an array of tuples of (<regular expression>, <replacement>)
static INFLECTION_RULES: LazyLock<Vec<(Regex, String)>> = LazyLock::new(|| {
    vec![("s$", "s"), ("$", "s")]
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

pub fn plural<S>(s: S) -> String
where
    S: Into<String>,
{
    let s: String = s.into();
    for (rule, replacement) in INFLECTION_RULES.iter() {
        if rule.is_match(&s) {
            let captures = rule.captures_iter(&s);
            for capture in captures {
                println!("{}", replacement);
                println!("{:?}", capture);
                let mut plural = String::new();
                capture.expand(replacement, &mut plural);
                println!("plural: {}", plural);
                return plural;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(plural("bell"), "bells");
    }
}
