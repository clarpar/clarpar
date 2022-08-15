pub enum Error {
    QuotedParamNotFollowedByWhitespaceChar,
    NoCodeAfterOptionAnnouncer,
    NoMatchSupportsValueForOptionCode,
    UnexpectedOptionValueAnnounced,
    QuotedOptionValueNotFollowedByWhitespaceChar,
    InvalidEscapedCharacterInOptionValue,
    ParamMissingClosingQuoteCharacter,
    InvalidEscapedCharacterInParam,
    OptionValueMissingClosingQuoteCharacter,
    ZeroLengthOptionCode,
    OptionCodeMissingDoubleAnnouncer,
    OptionValueCannotBeginWithOptionAnnouncer,
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
            Error::QuotedParamNotFollowedByWhitespaceChar => "Quoted parameter not followed by whitespace character",
            Error::NoCodeAfterOptionAnnouncer => "No code after option announcer",
            Error::NoMatchSupportsValueForOptionCode => "No match supports value for option code",
            Error::UnexpectedOptionValueAnnounced => "Unexpected option value announced",
            Error::QuotedOptionValueNotFollowedByWhitespaceChar => "Quoted option value not followed by whitespace character",
            Error::InvalidEscapedCharacterInOptionValue => "Invalid escaped character in option value",
            Error::ParamMissingClosingQuoteCharacter => "Parameter missing closing quote character",
            Error::InvalidEscapedCharacterInParam => "Invalid escaped character in parameter",
            Error::OptionValueMissingClosingQuoteCharacter => "Option value missing closing quote character",
            Error::ZeroLengthOptionCode => "Zero length option code",
            Error::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            Error::OptionValueCannotBeginWithOptionAnnouncer => "Option value cannot begin with option announcer. Try enclosing value in quotes",
            Error::UnmatchedOption => "Option not matched",
            Error::UnmatchedParam => "Parameter not matched",
        }
    }
}
