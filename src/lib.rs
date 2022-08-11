#![allow(clippy::collapsible_else_if)]

use regex::{Regex, RegexBuilder};

pub mod error;
enum ParseState {
    NotInArg,
    InParam,
    InParamPossibleEndQuote,
    InOption,
}

enum ParseOptionState {
    Announced,
    InCode,
    ValuePossible,
    ValueAnnounced,
    InValue,
    InValuePossibleEndQuote
}

pub const DEFAULT_QUOTE_CHAR: char = '"';
pub const DEFAULT_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
pub const DEFAULT_OPTION_CODES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
pub const DEFAULT_OPTION_VALUE_ANNOUNCER_CHAR: char = ' ';
pub const DEFAULT_PARSE_TERMINATE_CHARS: [char; 3] = ['<', '>', '|'];

#[derive(Default)]
pub struct RegexOrText {
    text: String,
    is_regex: bool,
    uppercase_text: Option<String>,
    regex: Option<Regex>,
    case_insensitive_regex: Option<Regex>,
}

impl RegexOrText {
    pub fn new_text(text: &str) -> Self {
        let mut result = Self {
            text: String::from(text),
            is_regex: false,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    pub fn new_regex(text: &str) -> Self {
        let mut result = Self {
            text: String::from(text),
            is_regex: true,
            ..RegexOrText::default()
        };

        result.update();

        result
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, value: &str) {
        self.text = String::from(value);
        self.is_regex = false;
        self.update();
    }

    pub fn get_uppercase_text(&self) -> &Option<String> {
        &self.uppercase_text
    }

    pub fn get_is_regex(&self) -> bool {
        self.regex.is_some()
    }

    pub fn get_regex(&self) -> &Option<Regex> {
        &self.regex
    }

    pub fn set_regex(&mut self, value: Regex) {
        self.text = String::from(value.as_str());
        self.is_regex = true;
        self.update();
    }

    pub fn is_match(&self, value: &str, case_sensitive: bool) -> bool {
        if self.is_regex {
            let optioned_regex_ref = if case_sensitive {
                self.regex.as_ref()
            } else {
                self.case_insensitive_regex.as_ref()
            };

            optioned_regex_ref.unwrap().is_match(value)
        } else {
            if case_sensitive {
                value.eq(&self.text)
            } else {
                let uppercase_value = self.to_uppercase(value);
                uppercase_value.eq(self.uppercase_text.as_ref().unwrap())
            }
        }
    }

    fn update(&mut self) {
        if self.is_regex {
            self.uppercase_text = None;

            self.regex = Some(Regex::new(&self.text).unwrap());

            let mut regex_builder = RegexBuilder::new(&self.text);
            regex_builder.case_insensitive(true);
            self.case_insensitive_regex = Some(regex_builder.build().unwrap());
        } else {
            self.regex = None;
            self.case_insensitive_regex = None;

            let mut uppercase_text = String::with_capacity(self.text.len());

            for char in self.text.chars() {
                uppercase_text.push(char);
            }
            self.uppercase_text = Some(self.to_uppercase(&self.text));
        }
    }

    fn to_uppercase(&self, value: &str) -> String {
        let mut result = String::with_capacity(self.text.len());

        for char in value.chars() {
            result.push(char);
        }

        result
    }
}

pub type DefaultTagType = usize;
pub struct Matcher<O = DefaultTagType, P = DefaultTagType> {
    pub name: String,
    pub help: Option<String>,
    pub option_tag: Option<O>,
    pub param_tag: Option<P>,
    // filters
    pub arg_indices: Option<Vec<usize>>,
    pub is_option: Option<bool>,
    pub option_indices: Option<Vec<usize>>,
    pub option_codes: Option<Vec<RegexOrText>>,
    pub option_has_value: Option<bool>,
    pub param_indices: Option<Vec<usize>>,
    pub value_text: Option<RegexOrText>,
}

impl<O, P> Matcher<O, P> {
    // pub fn get_arg_index(&self) ->Option<usize> {
    //     self.arg_index
    // }

    // pub fn set_arg_index(&self, value: Option<usize>) {
    //     self.arg_index = value;
    // }

    // pub fn get_option_index(&self) -> Option<usize> {
    //     self.option_index
    // }

    // pub fn set_option_index(&self, value: Option<usize>) {
    //     self.option_index = value;
    // }

    // pub fn get_option_codes(&self) -> Option<Vec<RegexOrText>> {
    //     self.option_codes
    // }

    // pub fn set_option_codes(&self, value: Option<Vec<RegexOrText>>) {
    //     self.option_codes = value;
    // }

    // pub fn get_option_has_value(&self) -> Option<bool> {
    //     self.option_has_value
    // }

    // pub fn set_option_has_value(&self, value: Option<bool>) {
    //     self.option_has_value = value;
    // }

    // pub fn get_param_index(&self) -> Option<usize> {
    //     self.param_index
    // }

    // pub fn set_param_index(&self, value: Option<usize>) {
    //     self.param_index = value;
    // }

    // pub fn get_value_text(&self) -> Option<RegexOrText> {
    //     self.value_text
    // }

    // pub fn set_value_text(&self, value: Option<RegexOrText>) {
    //     self.value_text = value;
    // }

    pub fn new(name: String) -> Self {
        Matcher {
            name,
            ..Default::default()
        }
    }

    pub fn matches_option_code(&self, value: &str, case_sensitive: bool) -> bool {
        let option_codes = self.option_codes.as_ref();
        match option_codes {
            None => true,
            Some(codes) => {
                for code in codes {
                    if code.is_match(value, case_sensitive) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

impl<O, P> Default for Matcher<O, P> {
    fn default() -> Self {
        Matcher {
            name: String::from(""),
            help: None,
            option_tag: None,
            param_tag: None,
            arg_indices: None,
            is_option: None,
            option_indices: None,
            option_codes: None,
            option_has_value: None,
            param_indices: None,
            value_text: None
        }
    }
}

pub type Matchers<O, P> = Vec<Matcher<O, P>>;

pub trait ArgProperties<O, P> {
    fn get_matcher(&self) -> &Matcher<O, P>;
    fn get_line_char_index(&self) -> usize;
    fn get_arg_index(&self) -> usize;
}

pub struct OptionProperties<'a, O, P> {
    pub matcher: &'a Matcher<O, P>,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub option_index: usize,
    pub code: String,
    pub value_text: Option<String>,
}

impl<'a, O, P> ArgProperties<O, P> for OptionProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_line_char_index(&self) -> usize {
        self.line_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
}

pub struct ParamProperties<'a, O, P> {
    pub matcher: &'a Matcher<O, P>,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub param_index: usize,
    pub value_text: String,
}

impl<'a, O, P> ArgProperties<O, P> for ParamProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_line_char_index(&self) -> usize {
        self.line_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
}

pub enum Arg<'a, O, P> {
    Param(ParamProperties<'a, O, P>),
    Option(OptionProperties<'a, O, P>),
}

pub type Args<'a, O, P> = Vec<Arg<'a, O, P>>;

struct ProcessCharResult<'a, O, P> {
    pub ignore_rest_of_line: bool,
    pub parsed_arg: Option<Arg<'a, O, P>>,
}

struct Session {
    multi_char_option_code_requires_double_announcer: bool,
    option_termination_chars: Vec<char>,
    line_len: usize,
    parse_state: ParseState,
    parse_option_state: ParseOptionState,
    arg_line_char_idx: usize,
    start_idx: usize,
    option_announcer_char: char,
    option_code: String,
    value_quoted: bool,
    value_bldr: String,
    arg_count: usize,
    option_count: usize,
    param_count: usize,
}

impl Session {
    fn set_option_code(& mut self, line: &str, optional_ending_index: Option<usize>) -> Result<(), String> {
        let ending_index = optional_ending_index.unwrap_or(self.line_len);
        let raw_option_code = &line[self.start_idx..ending_index];

        let mut raw_option_iterator = raw_option_code.chars();
        let optional_first_char = raw_option_iterator.next();
        match optional_first_char {
            None => {
                let error_text = error::Id::ZeroLengthOptionCode.to_text(Some(&self.arg_count.to_string()));
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
                            let error_text = error::Id::ZeroLengthOptionCode.to_text(Some(&self.arg_count.to_string()));
                            Err(error_text)
                        } else {
                            self.option_code = String::from(raw_option_code);
                            Ok(())
                        }
                    } else {
                        if !first_char_is_announcer {
                            let extra = format!("Arg: {} Option:\"{}\"", self.arg_count, raw_option_code);
                            let error_text = error::Id::OptionCodeMissingDoubleAnnouncer.to_text(Some(&extra));
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

pub struct Parser<O = DefaultTagType, P = DefaultTagType> {
    quote_char: char,
    option_announcer_chars: Vec<char>,
    option_codes_case_sensitive: bool,
    multi_char_option_code_requires_double_announcer: bool,
    option_value_announcer_char: char,
    parse_terminate_chars: Vec<char>,

    option_value_announcer_is_white_space: bool,

    matchers: Matchers<O, P>,
    fallback_matcher: Option<Matcher<O, P>>,
}

impl<O, P> Parser<O, P> {
    pub fn new() -> Self {
        Parser {
            quote_char: DEFAULT_QUOTE_CHAR,
            option_announcer_chars: DEFAULT_OPTION_ANNOUNCER_CHARS.to_vec(),
            option_codes_case_sensitive: DEFAULT_OPTION_CODES_CASE_SENSITIVE,
            multi_char_option_code_requires_double_announcer: DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
            option_value_announcer_char: DEFAULT_OPTION_VALUE_ANNOUNCER_CHAR,
            parse_terminate_chars: DEFAULT_PARSE_TERMINATE_CHARS.to_vec(),
            option_value_announcer_is_white_space: DEFAULT_OPTION_VALUE_ANNOUNCER_CHAR.is_whitespace(),
            matchers: Matchers::new(),
            fallback_matcher: Some(Matcher::new(String::from(""))),
        }
    }
}

impl<O, P> Default for Parser<O, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O, P> Parser<O, P> {

    /// Returns the character which can be used to enclose all text in a parameter or an option value.
    ///
    /// Spaces are used to delimit arguments in a command line.  If a parameter or an option value contains spaces,
    /// place quote_char at either end of the parameter or value text.  If the parameter or option value already contain 
    /// one or more quote character, then replace each quote character with 2 quote characters before placing a quote_char at either end
    /// of the text.
    ///
    /// You need to enclose a parameter or option value with quote_chars if the text starts with a quote character.  Make sure you
    /// replace the existing quote characters with 2 quote characters if the string is enclosed.
    ///
    /// Default: `"` (Double Quote character)
    pub fn quote_char(&self) -> &char {
        &self.quote_char
    }

    /// Sets the character used to enclose all text in a parameter or a option parameter value.
    ///
    /// See [`quote_char`](Parser::quote_char) documentation for more details.
    pub fn set_quote_char(&mut self, value: char) {
        self.quote_char = value;
    }

    /// Returns the array of characters any of which can be used to signify the start of an option argument in the command line.
    ///
    /// Any command line argument which begins with one of the characters in this array will be parsed as a option.
    ///
    /// Default: `-` (Dash character is the only character in the array)
    pub fn option_announcer_chars(&self) -> &Vec<char> {
        &self.option_announcer_chars
    }

    /// Sets the array of characters any of which can be used to signify the start of an option argument in the command line.
    ///
    /// See [`option_announcer_chars`](Parser::option_announcer_chars) documentation for more details.
    pub fn set_option_announcer_chars(&mut self, value: Vec<char>) {
        self.option_announcer_chars = value;
    }

    /// Returns the character which separates an option code from its value.
    ///
    /// If an option argument does not contain this character, then it is a switch/flag only and does not include a value.
    /// If it does contain this character, then the characters prior to this character are the option code and the characters after
    /// it, are the option value.
    ///
    /// Default: ` `  (Space character)
    pub fn option_value_announcer_char(&self) -> &char {
        &self.option_value_announcer_char
    }

    /// Sets the character which separates an option code from its value.
    ///
    /// See [`option_value_announcer_chars`](Parser::option_value_announcer_chars) documentation for more details.
    pub fn set_option_value_announcer_char(&mut self, value: char) {
        self.option_value_announcer_char = value;
        self.option_value_announcer_is_white_space = self.option_value_announcer_char.is_whitespace();
    }

    /// Returns an array of characters which terminate the parsing of arguments in the command line.
    /// 
    /// If any of the characters in this array are encountered outside a quoted value, then that character
    /// and all remaining characters in the command line are ignored.  This can be used to ignore standard input/output
    /// redirection and the end of a command line.
    ///
    /// Default: `<>|`  (standard input redirection to file, standard output redirection to file, pipe standard output)
    pub fn parse_terminate_chars(&self) -> &Vec<char> {
        &self.parse_terminate_chars
    }

    /// Sets the array of characters which terminate parsing of characters in the command line.
    ///
    /// See [`parse_terminate_chars`](Parser::parse_terminate_chars) documentation for more details.
    pub fn set_parse_terminate_chars(&mut self, value: Vec<char>) {
        self.parse_terminate_chars = value;
    }

    pub fn get_matchers(&self) -> &Matchers<O, P> {
        &self.matchers
    }

    pub fn add_matcher(&mut self, value: Matcher<O, P>) {
        self.matchers.push(value);
    }

    pub fn remove_matcher(&mut self, index: usize) {
        self.matchers.remove(index);
    }

    pub fn parse(&self, line: &str) -> Result<Args<O, P>, String> {
        let mut session = Session {
            multi_char_option_code_requires_double_announcer: self.multi_char_option_code_requires_double_announcer,
            line_len: line.chars().count(),
            parse_state: ParseState::NotInArg,
            parse_option_state: ParseOptionState::Announced,
            arg_line_char_idx: 0,
            start_idx: 0, // -1
            option_announcer_char: '\0',
            option_code: String::from(""),
            value_quoted: false,
            value_bldr: String::with_capacity(30),
            option_termination_chars: self.create_option_termination_char_array(),
            arg_count: 0,
            option_count: 0,
            param_count: 0,
        };

        let mut args: Args<O, P> = Vec::new();

        let mut char_idx = 0;
        for char in line.chars() {
            let process_char_result = self.process_char(&mut session, line, char_idx, char)?;
            if let Some(parsed_arg) = process_char_result.parsed_arg {
                args.push(parsed_arg);
            }

            if process_char_result.ignore_rest_of_line {
                break;
            } else {
                char_idx += 1;
            }
        }

        match session.parse_state {
            ParseState::NotInArg => {

            }

            ParseState::InParam => {
                if session.value_quoted {
                    self.create_error(error::Id::TextMissingClosingQuoteCharacter, None)?;
                } else {
                    let arg = self.process_param(&mut session)?;
                    args.push(arg);
                }
            }

            ParseState::InParamPossibleEndQuote => {
                let arg = self.process_param(&mut session)?;
                args.push(arg);
            }

            ParseState::InOption => {
                match session.parse_option_state {
                    ParseOptionState::Announced => {
                        self.create_error(error::Id::OptionNotSpecifiedAtLinePosition, Some(&session.line_len.to_string()))?;
                    }
                    ParseOptionState::InCode => {
                        session.set_option_code(line, None)?;
                        let arg = self.process_option(&mut session, false)?;
                        args.push(arg);
                    }
                    ParseOptionState::ValuePossible => {
                        let arg = self.process_option(&mut session, false)?;
                        args.push(arg);
                    }
                    ParseOptionState::ValueAnnounced => {
                        let arg = self.process_option(&mut session,  true)?;
                        args.push(arg);
                    }
                    ParseOptionState::InValue => {
                        if session.value_quoted {
                            self.create_error(error::Id::OptionMissingClosingQuoteCharacter, Some(&session.option_code))?;
                        } else {
                            let arg = self.process_option(&mut session, false)?;
                            args.push(arg);
                        }
                    }
                    ParseOptionState::InValuePossibleEndQuote => {
                        let arg = self.process_option(&mut session, false)?;
                        args.push(arg);
                    }
                }
            }
        }

        Ok(args)
    }

    fn process_char(&self, session: &mut Session, line: &str, char_idx: usize, line_char: char) -> Result<ProcessCharResult<O, P>, String> {
        let mut ignore_rest_of_line = false;
        let mut optional_parsed_arg: Option<Arg<O, P>> = None;

        match session.parse_state {
            ParseState::NotInArg => {
                if line_char == self.quote_char {
                    session.parse_state = ParseState::InParam;
                    session.arg_line_char_idx = char_idx;
                    session.start_idx = char_idx;
                    session.value_bldr.clear();
                    session.value_quoted = true;
                } else {
                    if self.option_announcer_chars.contains(&line_char) {
                        session.parse_state = ParseState::InOption;
                        session.parse_option_state = ParseOptionState::Announced;
                        session.option_announcer_char = line_char;
                        session.arg_line_char_idx = char_idx;
                        session.start_idx = char_idx + 1;
                    } else {
                        if self.parse_terminate_chars.contains(&line_char) {
                            ignore_rest_of_line = true;
                        } else {
                            if !line_char.is_whitespace() {
                                session.parse_state = ParseState::InParam;
                                session.arg_line_char_idx = char_idx;
                                session.start_idx = char_idx;
                                session.value_bldr.clear();
                                session.value_bldr.push(line_char);
                                session.value_quoted = false;
                            }
                        }
                    }
                }
            }

            ParseState::InParam => {
                if !session.value_quoted {
                    if !line_char.is_whitespace() {
                        session.value_bldr.push(line_char);
                    } else {
                        let parsed_arg = self.process_param(session)?;
                        optional_parsed_arg = Some(parsed_arg);
                        session.parse_state = ParseState::NotInArg;
                    }
                } else {
                    if line_char != self.quote_char {
                        session.value_bldr.push(line_char);
                    } else {
                        session.parse_state = ParseState::InParamPossibleEndQuote;
                    }
                }
            }

            ParseState::InParamPossibleEndQuote => {
                if line_char == self.quote_char {
                    session.value_bldr.push(line_char);
                    session.parse_state = ParseState::InParam;
                } else {
                    if line_char.is_whitespace() {
                        let parsed_arg = self.process_param(session)?;
                        optional_parsed_arg = Some(parsed_arg);
                        session.parse_state = ParseState::NotInArg;
                    } else {
                        self.create_error(error::Id::InvalidQuoteCharacterInTextParameter, Some(&session.value_bldr))?;
                    }
                }
            }

            ParseState::InOption => {
                match session.parse_option_state {
                    ParseOptionState::Announced => {
                        if line_char.is_whitespace() || session.option_termination_chars.contains(&line_char) {
                            self.create_error(error::Id::OptionNotSpecifiedAtLinePosition, Some(&char_idx.to_string()))?;
                        } else {
                            session.parse_option_state = ParseOptionState::InCode;
                        }
                    }
                    ParseOptionState::InCode => {
                        let option_value_announced = line_char == self.option_value_announcer_char;
                        if option_value_announced || line_char.is_whitespace() || session.option_termination_chars.contains(&line_char) {
                            session.set_option_code(line, Some(char_idx))?;
                            if session.option_code.is_empty() {
                                self.create_error(error::Id::OptionNotSpecifiedAtLinePosition, Some(&char_idx.to_string()))?;
                            } else {
                                if option_value_announced {
                                    if self.option_value_announcer_is_white_space {
                                        session.parse_option_state = ParseOptionState::ValuePossible;
                                    } else {
                                        session.parse_option_state = ParseOptionState::ValueAnnounced;
                                    }
                                } else {
                                    let parsed_arg = self.process_option(session, false)?;
                                    optional_parsed_arg = Some(parsed_arg);
                                    session.parse_state = ParseState::NotInArg;
                                }
                            }
                        }
                    }
                    ParseOptionState::ValuePossible => {
                        if !line_char.is_whitespace() {
                            if
                                !self.option_announcer_chars.contains(&line_char) // not a new option
                                &&
                                self.can_option_have_value(session) // option can have values
                            {
                                session.parse_option_state = ParseOptionState::ValueAnnounced;
                                return self.process_char(session, line, char_idx, line_char); // process first char of value
                            } else {
                                let parsed_arg = self.process_option(session, false)?; // process current option
                                optional_parsed_arg = Some(parsed_arg);
                                session.parse_state = ParseState::NotInArg;
                                let process_char_result = self.process_char(session, line, char_idx, line_char)?; // will handle new option/text param
                                ignore_rest_of_line = process_char_result.ignore_rest_of_line; // only ignore_rest_of_line could have changed
                            }
                        }
                    }
                    ParseOptionState::ValueAnnounced => {
                        session.start_idx = char_idx;
                        session.value_bldr.clear();
                        if line_char.is_whitespace() {
                            let parsed_arg = self.process_option(session, true)?;
                            optional_parsed_arg = Some(parsed_arg);
                            session.parse_state = ParseState::NotInArg;
                        } else {
                            if line_char == self.quote_char {
                                session.value_quoted = true;
                            }else {
                                session.value_quoted = false;
                                session.value_bldr.push(line_char);
                            }
                            session.parse_option_state = ParseOptionState::InValue;
                        }
                    }
                    ParseOptionState::InValue => {
                        if !session.value_quoted {
                            if !line_char.is_whitespace() {
                                session.value_bldr.push(line_char);
                            } else {
                                let parsed_arg = self.process_option(session, true)?;
                                optional_parsed_arg = Some(parsed_arg);
                                session.parse_state = ParseState::NotInArg;
                            }
                        } else {
                            if line_char != self.quote_char {
                                session.value_bldr.push(line_char);
                            } else {
                                session.parse_option_state = ParseOptionState::InValuePossibleEndQuote;
                            }
                        }
                    }
                    ParseOptionState::InValuePossibleEndQuote => {
                        if line_char == self.quote_char {
                            session.value_bldr.push(line_char);
                            session.parse_option_state = ParseOptionState::InValue;
                        } else {
                            if line_char.is_whitespace() {
                                let parsed_arg = self.process_option(session, true)?;
                                optional_parsed_arg = Some(parsed_arg);
                                session.parse_state = ParseState::NotInArg;
                            } else {
                                self.create_error(error::Id::InvalidQuoteCharacterInOptionValue, Some(&session.option_code))?;
                            }
                        }
                    }
                }
            }
        }

        Ok(ProcessCharResult { ignore_rest_of_line, parsed_arg: optional_parsed_arg })
    }

    fn create_option_termination_char_array(&self) -> Vec<char> {
        let mut result = Vec::with_capacity(2 + self.parse_terminate_chars.len());
        result.push(self.quote_char);
        result.push(self.option_value_announcer_char);
        result.extend(&self.parse_terminate_chars);

        result
    }

    fn can_option_have_value(&self, session: &Session) -> bool {
        let optioned_matcher = self.try_find_option_matcher(session, None);
        optioned_matcher.is_some()
    }

    fn process_option(&self, session: &mut Session, has_value: bool) -> Result<Arg<O, P>, String> {
        let optioned_matcher = self.try_find_option_matcher(session, Some(has_value));
        if let Some(matcher) = optioned_matcher {
            let value_text = if has_value {
                Some(session.value_bldr.clone())
            } else {
                None
            };
            let properties = OptionProperties {
                matcher,
                line_char_index: session.arg_line_char_idx,
                arg_index: session.arg_count,
                option_index: session.option_count,
                code: session.option_code.clone(),
                value_text
            };

            session.arg_count += 1;
            session.option_count += 1;

            let option_arg = Arg::Option(properties);
            Ok(option_arg)
        } else {
            self.create_error(error::Id::UnmatchedOption, Some(&session.option_code))
        }
    }

    fn try_find_option_matcher(&self, session: &Session, has_value: Option<bool>) -> Option<&Matcher<O, P>> {
        if self.matchers.is_empty() {
            self.fallback_matcher.as_ref()
        } else {
            self.matchers.iter().find(|&matcher| self.try_match_option(session, has_value, matcher))
        }
    }

    fn try_match_option(&self, session: &Session, has_value: Option<bool>, matcher: &Matcher<O, P>) -> bool {
        if  self.try_match_index(&session.arg_count, &matcher.arg_indices)
            &&
            self.try_match_bool(true, matcher.is_option)
            &&
            self.try_match_index(&session.option_count, &matcher.option_indices)
            &&
            self.try_match_option_code(&session.option_code, &matcher.option_codes)
        {
            if let Some(unwrapped_has_value) = has_value {
                // want to match value
                if let Some(unwrapped_matcher_option_has_value) = matcher.option_has_value {
                    // matcher explicitly specifies whether option should have value
                    if unwrapped_matcher_option_has_value {
                        // matcher expects value
                        if unwrapped_has_value {
                            // option has value - try match
                            self.try_match_value_text(&session.value_bldr, &matcher.value_text)
                        } else {
                            // option does not have value
                            false
                        }
                    } else {
                        // option does not expect value - return false if has value
                        !unwrapped_has_value
                    }
                } else {
                    // matcher specifies that option either can or cannot have value
                    if unwrapped_has_value {
                        // option has value - try match
                        self.try_match_value_text(&session.value_bldr, &matcher.value_text)
                    } else {
                        // option does not have value
                        true
                    }
                }
            } else {
                // only want to check if option can have value
                matcher.option_has_value.unwrap_or(true)
            }
            // if match_can_have_value {
            //     matcher.option_has_value.unwrap_or(true)
            // } else {
            //     if let Some(unwrapped_matcher_option_has_value) = matcher.option_has_value {
            //         if unwrapped_matcher_option_has_value {
            //             self.try_match_optioned_value_text(value_text, &matcher.value_text)
            //         } else {
            //             value_text.is_none()
            //         }
            //     } else {
            //         self.try_match_optioned_value_text(value_text, &matcher.value_text)
            //     }
            // }
        } else {
            false
        }
    }

    fn process_param(&self, session: &mut Session) -> Result<Arg<O, P>, String> {
        let optioned_matcher = if self.matchers.is_empty() {
            self.fallback_matcher.as_ref()
        } else {
            self.matchers.iter().find(|&matcher| self.try_match_param(session, matcher))
        };

        if let Some(matcher) = optioned_matcher {
            let properties = ParamProperties {
                matcher,
                line_char_index: session.arg_line_char_idx,
                arg_index: session.arg_count,
                param_index: session.param_count,
                value_text: session.value_bldr.clone(),
            };

            session.arg_count += 1;
            session.param_count += 1;

            let param_arg = Arg::Param(properties);
            Ok(param_arg)
        } else {
            self.create_error(error::Id::UnmatchedParam, Some(&session.arg_count.to_string()))
        }
    }


    fn try_match_param(&self, session: &Session, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&session.arg_count, &matcher.arg_indices)
        &&
        self.try_match_bool(false, matcher.is_option)
        &&
        self.try_match_value_text(&session.value_bldr, &matcher.value_text)
    }

    fn try_match_index(&self, index: &usize, matcher_indices: &Option<Vec<usize>>) -> bool {
        if let Some(unwrapped_matcher_indices) = matcher_indices {
            unwrapped_matcher_indices.contains(index)
        } else {
            true
        }
    }

    fn try_match_bool(&self, value: bool, matcher_value: Option<bool>) -> bool {
        if let Some(unwrapped_matcher_value) = matcher_value {
            unwrapped_matcher_value == value
        } else {
            true
        }
    }

    fn try_match_option_code(&self, code: &str, matcher_codes: &Option<Vec<RegexOrText>>) -> bool {
        if let Some(unwrapped_matcher_codes) = matcher_codes {
            for matcher_code in unwrapped_matcher_codes {
                if matcher_code.is_match(code, self.option_codes_case_sensitive) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    fn try_match_value_text(&self, value_text: &str, matcher_value_text: &Option<RegexOrText>) -> bool {
        if let Some(matcher_value_text) = matcher_value_text {
            matcher_value_text.is_match(value_text, self.option_codes_case_sensitive)
        } else {
            true
        }
    }

    fn create_error(&self, error_id: error::Id, extra: Option<&str>) -> Result<Arg<O, P>, String> {
        let error_text = error_id.to_text(extra);
        Err(error_text)
    }

}
