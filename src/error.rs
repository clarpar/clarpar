pub enum Error {
    InvalidQuoteCharacterInTextParameter,
    OptionNotSpecifiedAtLinePosition,
    InvalidQuoteCharacterInOptionValue,
    TextMissingClosingQuoteCharacter,
    OptionMissingClosingQuoteCharacter,
    ZeroLengthOptionCode,
    OptionCodeMissingDoubleAnnouncer,
    UnmatchedOption,
    UnmatchedParam,
}

impl Error {
    pub fn to_text(&self, optional_extra: Option<&str>) -> String {
        let error_text = self.get_default_text();
        let mut result = String::from(error_text);
        if let Some(extra) = optional_extra {
            result.push_str(": ");
            result.push_str(extra);
        }
        result
    }

    fn get_default_text(&self) -> &str {
        match self {
            Error::InvalidQuoteCharacterInTextParameter => "todo",
            Error::OptionNotSpecifiedAtLinePosition => "todo",
            Error::InvalidQuoteCharacterInOptionValue => "todo",
            Error::TextMissingClosingQuoteCharacter => "todo",
            Error::OptionMissingClosingQuoteCharacter => "todo",
            Error::ZeroLengthOptionCode => "Zero length option code",
            Error::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            Error::UnmatchedOption => "Option not matched",
            Error::UnmatchedParam => "Parameter not matched",
        }
    }
}
