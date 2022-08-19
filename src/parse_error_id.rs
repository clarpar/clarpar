use std::fmt::Display;

#[derive(Debug)]
pub enum ParseErrorId {
    QuotedParamNotFollowedByWhitespaceChar,
    NoMatchForOptionWithValue,
    UnexpectedOptionValueAnnounced,
    QuotedOptionValueNotFollowedByWhitespaceChar,
    EscapedCharacterInOptionValueCannotBeEscaped,
    ParamMissingClosingQuoteCharacter,
    EscapedCharacterInParamCannotBeEscaped,
    OptionValueMissingClosingQuoteCharacter,
    OptionCodeMissingDoubleAnnouncer,
    OptionMissingValue,
    OptionValueCannotBeginWithOptionAnnouncer,
    UnmatchedOption,
    UnmatchedParam,
}

impl ParseErrorId {
    pub fn get_default_text(&self) -> &str {
        match self {
            ParseErrorId::QuotedParamNotFollowedByWhitespaceChar => "Quoted parameter not followed by whitespace character",
            ParseErrorId::NoMatchForOptionWithValue => "No match for option with value",
            ParseErrorId::UnexpectedOptionValueAnnounced => "Unexpected option value announced",
            ParseErrorId::QuotedOptionValueNotFollowedByWhitespaceChar => "Quoted option value not followed by whitespace character",
            ParseErrorId::EscapedCharacterInOptionValueCannotBeEscaped => "Escaped character in option value cannot be escaped",
            ParseErrorId::ParamMissingClosingQuoteCharacter => "Parameter missing closing quote character",
            ParseErrorId::EscapedCharacterInParamCannotBeEscaped => "Escaped character in parameter cannot be escaped",
            ParseErrorId::OptionValueMissingClosingQuoteCharacter => "Option value missing closing quote character",
            ParseErrorId::OptionMissingValue => "Option missing value",
            ParseErrorId::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            ParseErrorId::OptionValueCannotBeginWithOptionAnnouncer => "Option value cannot begin with option announcer",
            ParseErrorId::UnmatchedOption => "Option not matched",
            ParseErrorId::UnmatchedParam => "Parameter not matched",
        }
    }
}

impl Display for ParseErrorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.get_default_text())
    }
}
