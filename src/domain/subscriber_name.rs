use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = s.chars().any(|g| forbidden_chars.contains(&g));

        println!(
            "2,{}, {},{},{}",
            s.clone(),
            is_empty_or_whitespace,
            is_too_long,
            contains_forbidden_chars
        );

        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid SubscriberName", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_graphme_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_is_rejected() {
        let name = "ë".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn white_space_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_container_an_invalid_character_are_rejected() {
        for name in &['/', '(', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_name_parsed_successfully() {
        let name = "ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
