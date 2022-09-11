use crate::parse_error_type_id::ParseErrorTypeId;
use std::{fmt::Display, error::Error};

/// Error result returned by a [Parser](crate::Parser) parse function ([parse_line](crate::Parser::parse_line),
/// [parse_env](crate::Parser::parse_env), [parse_env_args](crate::Parser::parse_env_args)) if it encounters a parse error.
#[derive(Debug)]
pub struct ParseError {
    /// The type of parse error
    pub type_id: ParseErrorTypeId,
    /// The index of the character in the command line where the error was detected. Note that if environment args are being parsed,
    /// this will be an approximate.
    pub line_char_index: usize,
    /// The index of the parsed argument in which either the error occurred or after which the error occurred.
    pub arg_index: usize,
    /// The index of the parsed option argument in which either the error occurred or after which the error occurred.
    pub option_index: Option<usize>,
    /// If an error occurred with an option argument, this will hold the option arguments code. Otherwise, it will contain `None`.
    pub option_code: Option<String>,
    /// The index of the parsed parameter argument in which either the error occurred or after which the error occurred.
    pub param_index: Option<usize>,
    /// If an error occurred with an parameter argument, this will hold the parameter value so far parsed. If the error occurred
    /// with an option argument, it will contain the option value so far parsed if the option has a value.  Otherwise it contains
    /// an empty string.
    pub param_value_text: String,
}

impl ParseError {
    pub (crate) fn new_option(type_id: ParseErrorTypeId, line_char_idx: usize, arg_idx: usize, option_idx: usize, option_code: &str, param_value_text: &str) -> Self {
        Self {
            type_id,
            line_char_index: line_char_idx,
            arg_index: arg_idx,
            option_index: Some(option_idx),
            option_code: Some(String::from(option_code)),
            param_index: None,
            param_value_text: String::from(param_value_text),
        }
    }

    pub (crate) fn new_param(type_id: ParseErrorTypeId, line_char_idx: usize, arg_idx: usize, param_idx: usize, param_value_text: &str) -> Self {
        Self {
            type_id,
            line_char_index: line_char_idx,
            arg_index: arg_idx,
            option_index: None,
            option_code: None,
            param_index: Some(param_idx),
            param_value_text: String::from(param_value_text),
        }
    }
}

impl Error for ParseError {
    
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_text = self.type_id.get_default_text();
        let mut error_text = String::with_capacity(id_text.len() + 60);
        error_text.push_str(" [l:");
        error_text.push_str(&self.line_char_index.to_string());
        error_text.push_str(" a:");
        error_text.push_str(&self.arg_index.to_string());

        if let Some(option_idx) = self.option_index {
            error_text.push_str(" o:");
            error_text.push_str(&option_idx.to_string());
            if let Some(option_code) = self.option_code.as_ref() {
                error_text.push_str(" c:");
                error_text.push_str(option_code);
                error_text.push('"');
            }
        } else {
            if let Some(param_idx) = self.param_index {
                error_text.push_str(" p:");
                error_text.push_str(&param_idx.to_string());
                error_text.push_str(" t:");
                error_text.push_str(&self.param_value_text);
                error_text.push('"');
            }
        }
        error_text.push(']');

        write!(f, "({})", error_text)
    }
}
