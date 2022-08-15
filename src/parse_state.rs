use crate::error;

#[derive(PartialEq, Eq)]
pub(crate) enum ArgParseState {
    NotInArg,
    InParam,
    InParamPossibleEndQuote,
    InParamEscaped,
    InOption,
}

#[derive(PartialEq, Eq)]
pub(crate) enum OptionParseState {
    Announced,
    InCode,
    WaitOptionValue,
    InValue,
    InValuePossibleEndQuote,
    InValueEscaped,
}

pub(crate) struct ParseState {
    pub(crate) quoting_active: bool,
    pub(crate) quote_char: char,
    pub(crate) multi_char_option_code_requires_double_announcer: bool,
    pub(crate) option_termination_chars: Vec<char>,
    pub(crate) line_len: usize,
    pub(crate) arg_parse_state: ArgParseState,
    pub(crate) option_parse_state: OptionParseState,
    pub(crate) arg_line_char_idx: usize,
    pub(crate) start_idx: usize,
    pub(crate) option_announcer_char: char,
    pub(crate) option_code: String,
    pub(crate) option_value_announcer_is_ambiguous: bool,
    pub(crate) current_option_value_may_be_param: bool,
    pub(crate) value_quoted: bool,
    pub(crate) value_bldr: String,
    pub(crate) arg_count: usize,
    pub(crate) option_count: usize,
    pub(crate) param_count: usize,
}

impl ParseState {
    pub(crate) fn set_option_code(& mut self, line: &str, optional_ending_index: Option<usize>) -> Result<(), String> {
        let ending_index = optional_ending_index.unwrap_or(self.line_len);
        let raw_option_code = &line[self.start_idx..ending_index];

        let mut raw_option_iterator = raw_option_code.chars();
        let optional_first_char = raw_option_iterator.next();
        match optional_first_char {
            None => {
                let error_text = error::Error::ZeroLengthOptionCode.to_text(Some(&self.arg_count.to_string()));
                Err(error_text)
            },
            Some(first_char) => {
                if !self.multi_char_option_code_requires_double_announcer {
                    self.option_code = String::from(raw_option_code);
                    Ok(())
                } else {
                    let first_char_is_announcer = first_char == self.option_announcer_char;
                    let announcer_is_one_char_only = raw_option_iterator.next() != None;
                    if announcer_is_one_char_only {
                        if first_char_is_announcer {
                            let error_text = error::Error::ZeroLengthOptionCode.to_text(Some(&self.arg_count.to_string()));
                            Err(error_text)
                        } else {
                            self.option_code = String::from(raw_option_code);
                            Ok(())
                        }
                    } else {
                        if !first_char_is_announcer {
                            let extra = format!("Arg: {} Option:\"{}\"", self.arg_count, raw_option_code);
                            let error_text = error::Error::OptionCodeMissingDoubleAnnouncer.to_text(Some(&extra));
                            Err(error_text)
                        } else {
                            self.option_code = String::from(raw_option_code);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}
