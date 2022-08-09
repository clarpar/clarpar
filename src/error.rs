pub enum Id {
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

impl Id {
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
            Id::InvalidQuoteCharacterInTextParameter => "todo",
            Id::OptionNotSpecifiedAtLinePosition => "todo",
            Id::InvalidQuoteCharacterInOptionValue => "todo",
            Id::TextMissingClosingQuoteCharacter => "todo",
            Id::OptionMissingClosingQuoteCharacter => "todo",
            Id::ZeroLengthOptionCode => "Zero length option code",
            Id::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            Id::UnmatchedOption => "Option not matched",
            Id::UnmatchedParam => "Parameter not matched",
        }
    }
}
