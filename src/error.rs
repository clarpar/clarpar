use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorId {
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

impl ErrorId {
    fn get_default_text(&self) -> &str {
        match self {
            ErrorId::QuotedParamNotFollowedByWhitespaceChar => "Quoted parameter not followed by whitespace character.",
            ErrorId::NoCodeAfterOptionAnnouncer => "No code after option announcer.",
            ErrorId::NoMatchSupportsValueForOptionCode => "No match supports value for option code.",
            ErrorId::UnexpectedOptionValueAnnounced => "Unexpected option value announced.",
            ErrorId::QuotedOptionValueNotFollowedByWhitespaceChar => "Quoted option value not followed by whitespace character.",
            ErrorId::InvalidEscapedCharacterInOptionValue => "Invalid escaped character in option value.",
            ErrorId::ParamMissingClosingQuoteCharacter => "Parameter missing closing quote character.",
            ErrorId::InvalidEscapedCharacterInParam => "Invalid escaped character in parameter.",
            ErrorId::OptionValueMissingClosingQuoteCharacter => "Option value missing closing quote character.",
            ErrorId::ZeroLengthOptionCode => "Zero length option code.",
            ErrorId::OptionCodeMissingDoubleAnnouncer => "Option code missing double announcer.",
            ErrorId::OptionValueCannotBeginWithOptionAnnouncer => "Option value cannot begin with option announcer. Try enclosing value in quotes.",
            ErrorId::UnmatchedOption => "Option not matched.",
            ErrorId::UnmatchedParam => "Parameter not matched.",
        }
    }
}

impl Display for ErrorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.get_default_text())
    }
}

#[derive(Debug)]
pub struct Error {
    id: ErrorId,
    line_char_idx: usize,
    arg_idx: usize,
    option_idx: Option<usize>,
    option_code: Option<String>,
    param_idx: Option<usize>,
    param_value_text: Option<String>,
}

impl Error {
    pub fn new(id: ErrorId, line_char_idx: usize, arg_idx: usize) -> Self {
        Self { id, line_char_idx, arg_idx, option_idx: None, option_code: None, param_idx: None, param_value_text: None }
    }

    pub fn new_option(id: ErrorId, line_char_idx: usize, arg_idx: usize, option_idx: usize, option_code: &str) -> Self {
        Self { id, line_char_idx, arg_idx, option_idx: Some(option_idx), option_code: Some(String::from(option_code)), param_idx: None, param_value_text: None }
    }

    pub fn new_param(id: ErrorId, line_char_idx: usize, arg_idx: usize, param_idx: usize, param_value_text: &str) -> Self {
        Self { id, line_char_idx, arg_idx, option_idx: None, option_code: None, param_idx: Some(param_idx), param_value_text: Some(String::from(param_value_text)) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_text = self.id.get_default_text();
        let mut error_text = String::with_capacity(id_text.len() + 60);
        error_text.push_str(" [l:");
        error_text.push_str(&self.line_char_idx.to_string());
        error_text.push_str(" a:");
        error_text.push_str(&self.arg_idx.to_string());

        if let Some(option_idx) = self.option_idx {
            error_text.push_str(" o:");
            error_text.push_str(&option_idx.to_string());
            if let Some(option_code) = self.option_code.as_ref() {
                error_text.push_str(" c:");
                error_text.push_str(option_code);
                error_text.push('"');
            }
        } else {
            if let Some(param_idx) = self.param_idx {
                error_text.push_str(" p:");
                error_text.push_str(&param_idx.to_string());
                if let Some(param_value_text) = self.param_value_text.as_ref() {
                    error_text.push_str(" t:");
                    error_text.push_str(param_value_text);
                    error_text.push('"');
                }
            }
        }
        error_text.push(']');

        write!(f, "({})", error_text)
    }
}
