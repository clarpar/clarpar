use std::fmt::Display;

#[derive(Debug)]
pub enum ParseErrorId {
    QuotedParamNotFollowedByWhitespaceChar,
    NoMatchForOptionWithValue,
    UnexpectedOptionValueAnnounced,
    QuotedOptionValueNotFollowedByWhitespaceChar,
    EscapedCharacterInOptionValueCannotBeEscaped,
    EscapeCharacterAtEndOfOptionValue,
    ParamMissingClosingQuoteCharacter,
    EscapedCharacterInParamCannotBeEscaped,
    EscapeCharacterAtEndOfParam,
    OptionCodeMissingDoubleAnnouncer,
    OptionCodeCannotContainQuoteChar,
    OptionCodeCannotContainEscapeChar,
    OptionMissingValue,
    OptionValueCannotBeginWithOptionAnnouncer,
    OptionValueMissingClosingQuoteCharacter,
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
            ParseErrorId::EscapeCharacterAtEndOfOptionValue => "Escape character is at end of option value",
            ParseErrorId::ParamMissingClosingQuoteCharacter => "Parameter missing closing quote character",
            ParseErrorId::EscapedCharacterInParamCannotBeEscaped => "Escaped character in parameter cannot be escaped",
            ParseErrorId::EscapeCharacterAtEndOfParam => "Escape character is at end of parameter",
            ParseErrorId::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            ParseErrorId::OptionCodeCannotContainQuoteChar => "Option code cannot contain quote character",
            ParseErrorId::OptionCodeCannotContainEscapeChar => "Option code cannot contain escape character",
            ParseErrorId::OptionValueCannotBeginWithOptionAnnouncer => "Option value cannot begin with option announcer",
            ParseErrorId::OptionMissingValue => "Option missing value",
            ParseErrorId::OptionValueMissingClosingQuoteCharacter => "Option value missing closing quote character",
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
