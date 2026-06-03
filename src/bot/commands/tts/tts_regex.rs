pub const MENTION_REGEX: &str = r"<@(\d+)>";
// regex struct -> http.://(subdomain-and-sld).(tld)(path, url query, etc... )
pub const LINK_REGEX: &str = r"<?https?://[[[:word:]]-@%.+~#=]{2,256}\.[[:alnum:]]{2,24}(?:[-[[:word:]]@:%+.~#?&/=\p{L}]*)>?";
pub const EMOJI_REGEX: &str = r"<a?:([a-zA-Z0-9_]+):\d+>";
pub const LAUGHTER_REGEX: &str = r"(?i)\b[jsadkf]{4,}\b";
pub const WHAT_REGEX: &str = r"(?i)\bq\b";
pub const WHY_REGEX: &str = r"(?i)\bxq\b";
pub const ALSO_REGEX: &str = r"(?i)\btmb\b";
pub const INLINE_CODE_BLOCK_REGEX: &str = r"`([^`\n]+)`";
pub const MULTI_LINE_DOUBLE_CODE_BLOCK_REGEX: &str = r"``([^`\n]+)``";
pub const MULTI_LINE_TRIPLE_CODE_BLOCK_REGEX: &str = r"```([\s\S]*?)```";
pub const CORRECTION_REGEX: &str = r"^\w+\*$";

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn aux_match(regex: &str, haystack: &str) -> bool {
        match Regex::new(regex) {
            Ok(re) => re.is_match(haystack),
            Err(_) => false,
        }
    }

    #[test]
    fn regex_definition() {
        assert!(Regex::new(LINK_REGEX).is_ok());
    }

    #[test]
    fn links_matchs() {
        let cases  = [
            "http://b32.i2p/",
            "http://a..xyz",
            "http://a..xyz",
            "http://...xyz",
            "<http://rustlang-es.org/",
            "http://rustlang-es.org/>",
            "https://rustlang-es.org/",
            "https://book.rustlang-es.org",
            "https://www.rustlang-es.org",
            "http://hi.xn--4gbrim/",
            "https://stackoverflow.com/questions/9238640/how-long-can-a-tld-possibly-be#:~:text=This%20answer%20is%20useful,to%20count%20the%20longest%20line"
        ];
        for case in cases {
            assert!(aux_match(LINK_REGEX, case));
        }
    }

    #[test]
    fn links_not_matchs() {
        let cases = [
            "ftp://b32.i2p/",
            "https://grüße.tld",
            "https://example.موقع",
        ];
        for case in cases {
            assert!(!aux_match(LINK_REGEX, case));
        }
    }
}
