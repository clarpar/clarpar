use crate::parse_error_id::{ParseErrorId};
use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub struct ParseError {
    pub id: ParseErrorId,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub option_index: Option<usize>,
    pub option_code: Option<String>,
    pub param_index: Option<usize>,
    pub param_value_text: Option<String>,
}

impl ParseError {
    pub (crate) fn new_option(id: ParseErrorId, line_char_idx: usize, arg_idx: usize, option_idx: usize, option_code: &str) -> Self {
        Self { id, line_char_index: line_char_idx, arg_index: arg_idx, option_index: Some(option_idx), option_code: Some(String::from(option_code)), param_index: None, param_value_text: None }
    }

    pub (crate) fn new_param(id: ParseErrorId, line_char_idx: usize, arg_idx: usize, param_idx: usize, param_value_text: &str) -> Self {
        Self { id, line_char_index: line_char_idx, arg_index: arg_idx, option_index: None, option_code: None, param_index: Some(param_idx), param_value_text: Some(String::from(param_value_text)) }
    }
}

impl Error for ParseError {
    
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_text = self.id.get_default_text();
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
