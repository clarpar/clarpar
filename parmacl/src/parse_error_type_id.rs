use std::fmt::Display;

/// The types of errors which can returned by the [Parser](crate::Parser) parse functions
#[derive(Debug)]
pub enum ParseErrorTypeId {
    /// A quoted parameter was not followed by a white space character.
    /// This can be caused by embedding a quote character within the quoted string.
    QuotedParamNotFollowedByWhitespaceChar,
    /// An option argument requires a value but no matcher found which supports both the option code
    /// and allows the option to have a value.
    NoMatchForOptionWithValue,
    /// A quoted option value was not followed by a white space character.
    /// This can be caused by embedding a quote character within the quoted string.
    QuotedOptionValueNotFollowedByWhitespaceChar,
    /// Escape character is at the end of line.
    EscapeCharacterAtEndOfLine,
    /// An escaped character in an option value cannot be escaped.
    EscapedCharacterInOptionValueCannotBeEscaped,
    /// Escape character is at the end of an option value.
    EscapeCharacterAtEndOfOptionValue,
    /// A quoted parameter argument is missing the closing quote character.
    ParamMissingClosingQuoteCharacter,
    /// An escaped character in a parameter cannot be escaped.
    EscapedCharacterInParamCannotBeEscaped,
    /// Escape character is at the end of a parameter.
    EscapeCharacterAtEndOfParam,
    /// Option code with length longer than 1 requires a double announcer.
    OptionCodeMissingDoubleAnnouncer,
    /// Option code cannot contain a quote character.
    OptionCodeCannotContainQuoteChar,
    /// Option code cannot contain an escape character.
    OptionCodeCannotContainEscapeChar,
    /// Option requires a value but value is missing.
    OptionMissingValue,
    /// Option value cannot start with option announcer character.
    OptionValueCannotStartWithOptionAnnouncer,
    /// A quoted option value is missing the closing quote character.
    OptionValueMissingClosingQuoteCharacter,
    /// No match found for option argument.
    UnmatchedOption,
    /// No match found for parameter argument.
    UnmatchedParam,
}

impl ParseErrorTypeId {
    /// Get the default (English) text which describes an error type.
    pub fn get_default_text(&self) -> &str {
        match self {
            ParseErrorTypeId::QuotedParamNotFollowedByWhitespaceChar => "Quoted parameter not followed by whitespace character",
            ParseErrorTypeId::NoMatchForOptionWithValue => "No match for option with value",
            ParseErrorTypeId::QuotedOptionValueNotFollowedByWhitespaceChar => "Quoted option value not followed by whitespace character",
            ParseErrorTypeId::EscapeCharacterAtEndOfLine => "Escape character is at end of line",
            ParseErrorTypeId::EscapedCharacterInOptionValueCannotBeEscaped => "Escaped character in option value cannot be escaped",
            ParseErrorTypeId::EscapeCharacterAtEndOfOptionValue => "Escape character is at end of option value",
            ParseErrorTypeId::ParamMissingClosingQuoteCharacter => "Parameter missing closing quote character",
            ParseErrorTypeId::EscapedCharacterInParamCannotBeEscaped => "Escaped character in parameter cannot be escaped",
            ParseErrorTypeId::EscapeCharacterAtEndOfParam => "Escape character is at end of parameter",
            ParseErrorTypeId::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer",
            ParseErrorTypeId::OptionCodeCannotContainQuoteChar => "Option code cannot contain quote character",
            ParseErrorTypeId::OptionCodeCannotContainEscapeChar => "Option code cannot contain escape character",
            ParseErrorTypeId::OptionValueCannotStartWithOptionAnnouncer => "Option value cannot start with option announcer",
            ParseErrorTypeId::OptionMissingValue => "Option missing value",
            ParseErrorTypeId::OptionValueMissingClosingQuoteCharacter => "Option value missing closing quote character",
            ParseErrorTypeId::UnmatchedOption => "Option not matched",
            ParseErrorTypeId::UnmatchedParam => "Parameter not matched",
        }
    }
}

impl Display for ParseErrorTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.get_default_text())
    }
}
