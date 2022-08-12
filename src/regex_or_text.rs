use regex::{Regex, RegexBuilder};

/// Specifies a text string or regex.
/// Used by to match option codes and values
/// 
/// If it contains a text string, then matcher will compare
#[derive(Default)]
pub struct RegexOrText {
    text: String,
    override_case_sensitive: Option<bool>,
    is_regex: bool,
    uppercase_text: Option<String>,
    regex: Option<Regex>,
    case_insensitive_regex: Option<Regex>,
}

impl RegexOrText {
    pub fn new_text(text: &str) -> Self {
        let mut result = Self {
            text: String::from(text),
            is_regex: false,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    pub fn new_regex(text: &str) -> Self {
        let mut result = Self {
            text: String::from(text),
            is_regex: true,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, value: &str) {
        self.text = String::from(value);
        self.is_regex = false;
        self.update();
    }

    pub fn get_uppercase_text(&self) -> &Option<String> {
        &self.uppercase_text
    }

    pub fn get_is_regex(&self) -> bool {
        self.regex.is_some()
    }

    pub fn get_regex(&self) -> &Option<Regex> {
        &self.regex
    }

    pub fn set_regex(&mut self, value: Regex) {
        self.text = String::from(value.as_str());
        self.is_regex = true;
        self.update();
    }

    pub fn get_override_case_sensitive(&self) -> Option<bool> {
        self.override_case_sensitive
    }

    pub fn set_override_case_sensitive(&mut self, value: Option<bool>) {
        self.override_case_sensitive = value;
    }

    pub fn is_match(&self, value: &str, mut case_sensitive: bool) -> bool {
        if self.is_regex {
            if let Some(override_case_sensitive) = self.override_case_sensitive {
                case_sensitive = override_case_sensitive;
            }

            let optioned_regex_ref = if case_sensitive {
                self.regex.as_ref()
            } else {
                self.case_insensitive_regex.as_ref()
            };

            optioned_regex_ref.unwrap().is_match(value)
        } else {
            if case_sensitive {
                value.eq(&self.text)
            } else {
                let uppercase_value = self.to_uppercase(value);
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
            self.uppercase_text = Some(self.to_uppercase(&self.text));
        }
    }

    fn to_uppercase(&self, value: &str) -> String {
        let mut result = String::with_capacity(self.text.len());

        for char in value.chars() {
            result.push(char);
        }

        result
    }
}

