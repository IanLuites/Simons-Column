//! Utility functions

/// Clean an Id by ensuring it's alphanumeric + '_' and the '_' does not repeat.
#[must_use]
pub fn clean_id(id: impl AsRef<str>) -> Option<String> {
    let mut prev_char = '\0';
    let slug = id
        .as_ref()
        .chars()
        .filter(|&c| {
            if c.is_ascii_alphanumeric() || (c == '_' && prev_char != '_') {
                prev_char = c;
                true
            } else {
                false
            }
        })
        .collect::<String>()
        .to_ascii_lowercase();

    if slug.is_empty() {
        None
    } else {
        Some(slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_clean_id() {
        assert_eq!(clean_id("Hello"), Some("hello".into()));
        assert_eq!(clean_id("My__name"), Some("my_name".into()));
        assert_eq!(clean_id(" My_ _nam e "), Some("my_name".into()));
        assert_eq!(clean_id("   "), None);
    }
}
