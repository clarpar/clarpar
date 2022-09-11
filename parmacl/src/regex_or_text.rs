use regex::{Regex, RegexBuilder};

/// Specifies a text string or regex.
/// Used in [Matcher](crate::Matcher) filters to match [parameters](crate::Matcher::value_text),
/// [option codes](crate::Matcher::option_codes) and [option values](crate::Matcher::value_text).
/// 
/// If it contains a text string, then the matcher filter will see if the corresponding argument property is
/// equal to the text.  If it contains a regex, then the matcher filter will see if the corresponding argument
/// property is a match for the regex.
/// 
/// Normally, whether or not the text comparison or regex matching is done with case sensitivity, is determined by
/// the relevant [Parser](crate::Parser) property: [params_case_sensitive](crate::Parser::params_case_sensitive),
/// [option_codes_case_sensitive](crate::Parser::option_codes_case_sensitive),
/// [option_values_case_sensitive](crate::Parser::option_values_case_sensitive).  However this can be overridden
/// with the `RegexOrText`'s [override_case_sensitive](Self::override_case_sensitive) property.
#[derive(Debug, Default, Clone)]
pub struct RegexOrText {
    text: String,
    override_case_sensitive: Option<bool>,
    is_regex: bool,
    uppercase_text: Option<String>,
    regex: Option<Regex>,
    case_insensitive_regex: Option<Regex>,
}

impl RegexOrText {
    /// Create a new `RegexOrText` which matches the specified text.
    pub fn with_text(text: &str) -> Self {
        let mut result = Self {
            text: String::from(text),
            is_regex: false,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    /// Create a new `RegexOrText` which matches the specified regex.
    pub fn with_regex(regex: Regex) -> Self {
        let mut result = Self {
            regex: Some(regex),
            is_regex: true,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    /// Get the text value of the `RegexOrText`.  Note that this will be ignored if the `RegexOrText` has been configured
    /// to match with a regex ([is_regex](Self::is_regex) returns true).
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the text used for matching. This will also configure the `RegexOrText` to match with text.
    pub fn set_text(&mut self, value: &str) -> &mut Self {
        self.text = String::from(value);
        self.is_regex = false;
        self.update();
        self
    }

    /// Get the uppercase value of the text value.  This will be used if case insensitive matching is configured.
    pub fn get_uppercase_text(&self) -> &Option<String> {
        &self.uppercase_text
    }

    /// Returns true if `RegexOrText` will match with a regex. Otherwise returns false (matches with text).
    pub fn is_regex(&self) -> bool {
        self.regex.is_some()
    }

    /// Get the optioned regex value of the `RegexOrText`.  Note that this will be ignored if the `RegexOrText` has been configured
    /// to match with text ([is_regex](Self::is_regex) returns false).
    pub fn regex(&self) -> &Option<Regex> {
        &self.regex
    }

    /// Sets the regex used for matching. This will also configure the `RegexOrText` to match with regex.
    pub fn set_regex(&mut self, value: Regex) -> &mut Self {
        self.text = String::from(value.as_str());
        self.is_regex = true;
        self.update();
        self
    }

    /// Gets an optioned boolean which indicates whether matching will be case sensitive.  If the value is `None` then matching
    /// will use the case sensitivity specified by the corresponding Parser property: 
    /// ([params_case_sensitive](crate::Parser::params_case_sensitive),
    /// [option_codes_case_sensitive](crate::Parser::option_codes_case_sensitive),
    /// [option_values_case_sensitive](crate::Parser::option_values_case_sensitive)). If value contains some boolean, then the value
    /// of the boolean indicates whether matching will be case sensitive.
    pub fn override_case_sensitive(&self) -> Option<bool> {
        self.override_case_sensitive
    }

    /// Set [override_case_sensitive](Self::override_case_sensitive).
    pub fn set_override_case_sensitive(&mut self, value: Option<bool>) -> &mut Self {
        self.override_case_sensitive = value;
        self
    }

    /// Test if value matches `RegexOrText`. The `case_sensitive` parameter specifies whether the matching is case senstive however
    /// note that this will be overridden if the [override_case_sensitive](Self::override_case_sensitive) property has some value.
    pub fn is_match(&self, value: &str, mut case_sensitive: bool) -> bool {
        if let Some(override_case_sensitive) = self.override_case_sensitive {
            case_sensitive = override_case_sensitive;
        }

        if self.is_regex {
            let optioned_regex_ref = if case_sensitive {
                self.regex.as_ref()
            } else {
                self.case_insensitive_regex.as_ref()
            };

            optioned_regex_ref.expect("Regex not set in RegexOrText").is_match(value)
        } else {
            if case_sensitive {
                value.eq(&self.text)
            } else {
                let uppercase_value = value.to_uppercase();
                uppercase_value.eq(self.uppercase_text.as_ref().unwrap())
            }
        }
    }

    fn update(&mut self) {
        if self.is_regex {
            self.uppercase_text = None;

            self.regex = Some(Regex::new(&self.text).unwrap());

            let mut regex_builder = RegexBuilder::new(&self.text);
            regex_builder.case_insensitive(true);
            self.case_insensitive_regex = Some(regex_builder.build().unwrap());
        } else {
            self.regex = None;
            self.case_insensitive_regex = None;

            let mut uppercase_text = String::with_capacity(self.text.len());

            for char in self.text.chars() {
                uppercase_text.push(char);
            }
            self.uppercase_text = Some(self.text.to_uppercase());
        }
    }
}

