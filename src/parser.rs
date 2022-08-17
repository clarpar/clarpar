use crate::error::{ErrorId, Error};
use crate::regex_or_text::{RegexOrText};
use crate::arg::{Arg, Args, OptionProperties, ParamProperties};
use crate::matcher::{Matcher, Matchers, OptionHasValue, DefaultTagType, DEFAULT_OPTION_HAS_VALUE, ArgType};
use crate::parse_state::{ParseState, ArgParseState, OptionParseState};

pub const DEFAULT_QUOTE_CHAR: Option<char> = Some('"');
pub const DEFAULT_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
pub const DEFAULT_OPTION_CODES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
pub const DEFAULT_OPTION_VALUE_ANNOUNCER_CHARS: [char; 1] = [' '];
pub const DEFAULT_OPTION_VALUES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_PARAMS_CASE_SENSITIVE: bool = false;
pub const DEFAULT_EMBED_QUOTE_CHAR_WITH_DOUBLE: bool = true;
pub const DEFAULT_ESCAPE_CHAR: Option<char> = None;
pub const DEFAULT_ONLY_ESCAPE_QUOTE_CHAR_AND_ESCAPE_CHAR: bool = true;
pub const DEFAULT_PARSE_TERMINATE_CHARS: [char; 3] = ['<', '>', '|'];

pub struct Parser<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    /// An `Option` specifying the character which can be used to enclose all text in a parameter or an option value. `None` specifies that
    /// quoting is not to be used.
    ///
    /// Whitespace characters (normally spaces) are used to delimit arguments in a command line.  If a parameter or an option value contain
    /// whitespace characters, place a quote_char at either end of the parameter or value text.  If the parameter or option value already contain 
    /// one or more quote characters, then these can be embedded using either [`Double quote characters`](Parser::embed_quote_char_with_double) or
    /// [`Escaping`](Parser::escape_char)
    ///
    /// If text starts with a quote character, you also need to embed it with either quoting or escaping.  You can also use quoting to enclose text
    /// which begins with a option announcer but is not an option.  See [`Matcher.option_has_value`](Matcher::option_has_value) for alternative
    /// ways of handling text beginning with the option announcer character.
    ///
    /// Default: `Some('"')` (Enabled using double quote character)
    pub quote_char: Option<char>,
    /// The array of characters any of which can be used to signify the start of an option argument in the command line.
    ///
    /// Normally a command line argument which begins with one of the characters in this array will be parsed as a option. However this behaviour
    /// can be overridden with [`Matcher.option_has_value`](Matcher::option_has_value).
    ///
    /// Default: `-` (Dash character is the only character in the array)
    pub option_announcer_chars: Vec<char>,
    pub option_codes_case_sensitive: bool,
    pub multi_char_option_code_requires_double_announcer: bool,
    /// The array of character any of which can be used end an option code and announce its option value.
    ///
    /// If an option argument does not end with this character, then it is a switch/flag only and does not include a value.
    /// If it does contain this character, then the characters prior to this character are the option code and the characters after
    /// it, are the option value.
    /// 
    /// Note that if a whitespace character is used as a option value announcer, there is some ambiguity as to whether that character is
    /// announcing the value for that option or being a delimiter for the next argument.  This ambiguity is resolved by a matcher's
    /// [`Matcher.option_has_value`](Matcher::option_has_value) property.
    ///
    /// Default: ` `  (Space character)
    pub option_value_announcer_chars: Vec<char>,
    pub option_values_case_sensitive: bool,
    pub params_case_sensitive: bool,
    pub embed_quote_char_with_double: bool,
    pub escape_char: Option<char>,
    pub only_escape_quote_char_and_escape_char: bool,
    /// An array of characters which terminate the parsing of arguments in the command line.
    /// 
    /// If any of the characters in this array are encountered outside a quoted value, then that character
    /// and all remaining characters in the command line are ignored.  This can be used to ignore standard input/output
    /// redirection and the end of a command line.
    ///
    /// Default: `<>|`  (standard input redirection to file, standard output redirection to file, pipe standard output)
    pub parse_terminate_chars: Vec<char>,

    matchers: Matchers<O, P>,
    fallback_matcher: Matcher<O, P>,
}

impl<O: Default, P: Default> Parser<O, P> {
    pub fn new() -> Self {
        Parser {
            quote_char: DEFAULT_QUOTE_CHAR,
            option_announcer_chars: DEFAULT_OPTION_ANNOUNCER_CHARS.to_vec(),
            option_codes_case_sensitive: DEFAULT_OPTION_CODES_CASE_SENSITIVE,
            multi_char_option_code_requires_double_announcer: DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
            option_value_announcer_chars: DEFAULT_OPTION_VALUE_ANNOUNCER_CHARS.to_vec(),
            option_values_case_sensitive: DEFAULT_OPTION_VALUES_CASE_SENSITIVE,
            params_case_sensitive: DEFAULT_PARAMS_CASE_SENSITIVE,
            embed_quote_char_with_double: DEFAULT_EMBED_QUOTE_CHAR_WITH_DOUBLE,
            escape_char: DEFAULT_ESCAPE_CHAR,
            only_escape_quote_char_and_escape_char: DEFAULT_ONLY_ESCAPE_QUOTE_CHAR_AND_ESCAPE_CHAR,
            parse_terminate_chars: DEFAULT_PARSE_TERMINATE_CHARS.to_vec(),

            matchers: Matchers::new(),
            fallback_matcher: Matcher::new(String::from("")),
        }
    }
}

impl<O: Default, P: Default> Default for Parser<O, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Default, P: Default> Parser<O, P> {

    pub fn get_matchers(&self) -> &Matchers<O, P> {
        &self.matchers
    }

    pub fn add_matcher(&mut self, value: Matcher<O, P>) {
        self.matchers.push(value);
    }

    pub fn remove_matcher(&mut self, index: usize) {
        self.matchers.remove(index);
    }

    pub fn clear_matchers(&mut self) {
        self.matchers.clear();
    }

    pub fn parse(&self, line: &str) -> Result<Args<O, P>, Error> {
        let mut args = Vec::new();

        let quoting_active = self.quote_char.is_some();
        let quote_char = self.quote_char.unwrap_or('\0');
        let mut parse_state = ParseState {
            quoting_active,
            quote_char,
            multi_char_option_code_requires_double_announcer: self.multi_char_option_code_requires_double_announcer,
            line_len: line.chars().count(),
            arg_parse_state: ArgParseState::NotInArg,
            option_parse_state: OptionParseState::Announced,
            line_char_idx: 0,
            arg_start_line_char_idx: 0,
            option_code_start_line_char_idx: 0,
            option_announcer_char: '\0',
            option_code: String::from(""),
            option_value_announcer_is_ambiguous: false,
            current_option_value_may_be_param: false,
            value_quoted: false,
            value_bldr: String::with_capacity(30),
            option_termination_chars: self.create_option_termination_char_array(quoting_active, quote_char),
            arg_count: 0,
            option_count: 0,
            param_count: 0,
        };

        for char in line.chars() {
            let more = self.process_char(& mut parse_state, line, char, &mut args)?;

            if more {
                parse_state.increment_line_char_idx();
            } else {
                // ignore rest of line
                break;
            }
        }

        match parse_state.arg_parse_state {
            ArgParseState::NotInArg => {

            }

            ArgParseState::InParam => {
                if parse_state.value_quoted {
                    Err(parse_state.create_param_error(ErrorId::ParamMissingClosingQuoteCharacter))?;
                } else {
                    self.match_param_arg(&mut parse_state, &mut args)?;
                }
            }

            ArgParseState::InParamPossibleEndQuote => {
                self.match_param_arg(&mut parse_state, &mut args)?;
            }

            ArgParseState::InParamEscaped => {
                Err(parse_state.create_param_error(ErrorId::InvalidEscapedCharacterInParam))?;
            }

            ArgParseState::InOption => {
                match parse_state.option_parse_state {
                    OptionParseState::Announced => {
                        Err(parse_state.create_option_error(ErrorId::NoCodeAfterOptionAnnouncer))?;
                    }
                    OptionParseState::InCode => {
                        parse_state.set_option_code(line, None)?;
                        self.match_option_arg(&mut parse_state, false, &mut args)?;
                    }
                    OptionParseState::WaitOptionValue => {
                        let has_value = self.can_option_have_value_with_first_char(&parse_state, false)?;
                        match has_value {
                            OptionHasValueBasedOnFirstChar::Must => {
                                Err(parse_state.create_option_error(ErrorId::NoMatchSupportsValueForOptionCode))?;
                            }
                            OptionHasValueBasedOnFirstChar::Possibly => {
                                parse_state.current_option_value_may_be_param = false;
                                self.match_option_arg(&mut parse_state, false, &mut args)?;
                            }
                            OptionHasValueBasedOnFirstChar::MustNot => {
                                parse_state.current_option_value_may_be_param = false;
                                self.match_option_arg(&mut parse_state, false, &mut args)?;
                            }
                        }
                    }
                    OptionParseState::InValue => {
                        if parse_state.value_quoted {
                            Err(parse_state.create_option_error(ErrorId::OptionValueMissingClosingQuoteCharacter))?;
                        } else {
                            self.match_option_arg(&mut parse_state, true, &mut args)?;
                        }
                    }
                    OptionParseState::InValuePossibleEndQuote => {
                        self.match_option_arg(&mut parse_state, true, &mut args)?;
                    }
                    OptionParseState::InValueEscaped => {
                        Err(parse_state.create_option_error(ErrorId::InvalidEscapedCharacterInOptionValue))?;
                    }
                }
            }
        }

        Ok(args)
    }

    fn process_char<'a>(&'a self, parse_state: &mut ParseState, line: &str, line_char: char, args: &mut Args<'a, O, P>) -> Result<bool, Error> {
        let mut more = true;

        match parse_state.arg_parse_state {
            ArgParseState::NotInArg => {
                if parse_state.quoting_active && line_char == parse_state.quote_char {
                    parse_state.arg_parse_state = ArgParseState::InParam;
                    parse_state.value_bldr.clear();
                    parse_state.arg_start_line_char_idx = parse_state.line_char_idx;
                    parse_state.value_quoted = true;
                } else {
                    if self.option_announcer_chars.contains(&line_char) {
                        parse_state.arg_parse_state = ArgParseState::InOption;
                        parse_state.option_parse_state = OptionParseState::Announced;
                        parse_state.option_announcer_char = line_char;
                        parse_state.arg_start_line_char_idx = parse_state.line_char_idx;
                        parse_state.option_code_start_line_char_idx = parse_state.line_char_idx + 1;
                    } else {
                        if self.parse_terminate_chars.contains(&line_char) {
                            more = false;
                        } else {
                            if !line_char.is_whitespace() {
                                parse_state.arg_parse_state = ArgParseState::InParam;
                                parse_state.value_bldr.clear();
                                parse_state.value_bldr.push(line_char);
                                parse_state.arg_start_line_char_idx = parse_state.line_char_idx;
                                parse_state.value_quoted = false;
                            }
                        }
                    }
                }
            }

            ArgParseState::InParam => {
                if let Some(unwrapped_escape_char) = self.escape_char {
                    if unwrapped_escape_char == line_char {
                        parse_state.arg_parse_state = ArgParseState::InParamEscaped;
                    }
                }

                if parse_state.arg_parse_state == ArgParseState::InParam {
                    if parse_state.value_quoted {
                        if line_char == parse_state.quote_char {
                            parse_state.arg_parse_state = ArgParseState::InParamPossibleEndQuote;
                        } else {
                            parse_state.value_bldr.push(line_char);
                        }
                    } else {
                        if !line_char.is_whitespace() {
                            parse_state.value_bldr.push(line_char);
                        } else {
                            self.match_param_arg(parse_state, args)?;
                            parse_state.arg_parse_state = ArgParseState::NotInArg;
                        }
                    }
                }
            }

            ArgParseState::InParamPossibleEndQuote => {
                if line_char == parse_state.quote_char && self.embed_quote_char_with_double {
                    parse_state.value_bldr.push(line_char);
                    parse_state.arg_parse_state = ArgParseState::InParam;
                } else {
                    if line_char.is_whitespace() {
                        self.match_param_arg(parse_state, args)?;
                        parse_state.arg_parse_state = ArgParseState::NotInArg;
                    } else {
                        Err(parse_state.create_param_error(ErrorId::QuotedParamNotFollowedByWhitespaceChar))?;
                    }
                }
            }

            ArgParseState::InParamEscaped => {
                let char_can_be_escaped: bool;
                if !self.only_escape_quote_char_and_escape_char {
                    char_can_be_escaped = true;
                } else {
                    if parse_state.quoting_active && line_char == parse_state.quote_char {
                        char_can_be_escaped = true;
                    } else {
                        if let Some(unwrapped_escape_char) = self.escape_char {
                            if line_char == unwrapped_escape_char {
                                char_can_be_escaped = true;
                            } else {
                                char_can_be_escaped = false;
                            }
                        } else {
                            char_can_be_escaped = false;
                        }
                    }
                }

                if char_can_be_escaped {
                    parse_state.value_bldr.push(line_char);
                    parse_state.arg_parse_state = ArgParseState::InParam;
                }
            }

            ArgParseState::InOption => {
                match parse_state.option_parse_state {
                    OptionParseState::Announced => {
                        if line_char.is_whitespace() || parse_state.option_termination_chars.contains(&line_char) {
                            Err(parse_state.create_option_error(ErrorId::NoCodeAfterOptionAnnouncer))?;
                        } else {
                            parse_state.option_parse_state = OptionParseState::InCode;
                        }
                    }
                    OptionParseState::InCode => {
                        let option_value_announced = self.option_value_announcer_chars.contains(&line_char);
                        let line_char_is_whitespace = line_char.is_whitespace();
                        if option_value_announced || line_char_is_whitespace || parse_state.option_termination_chars.contains(&line_char) {
                            parse_state.set_option_code(line, Some(parse_state.line_char_idx))?;
                            if parse_state.option_code.is_empty() {
                                Err(parse_state.create_option_error(ErrorId::NoCodeAfterOptionAnnouncer))?;
                            } else {
                                if option_value_announced {
                                    parse_state.option_value_announcer_is_ambiguous = line_char_is_whitespace;
                                    if self.can_option_code_have_value(parse_state) {
                                        parse_state.option_parse_state = OptionParseState::WaitOptionValue;
                                    } else {
                                        if parse_state.option_value_announcer_is_ambiguous {
                                            parse_state.current_option_value_may_be_param = false;
                                            self.match_option_arg(parse_state, false, args)?;
                                            parse_state.arg_parse_state = ArgParseState::NotInArg;
                                        } else {
                                            Err(parse_state.create_option_error(ErrorId::NoMatchSupportsValueForOptionCode))?;
                                        }
                                    }
                                } else {
                                    parse_state.current_option_value_may_be_param = false;
                                    self.match_option_arg(parse_state, false, args)?;
                                    parse_state.arg_parse_state = ArgParseState::NotInArg;
                                }
                            }
                        }
                    }
                    OptionParseState::WaitOptionValue => {
                        if !line_char.is_whitespace() {
                            let first_char_of_value_is_option_announcer = self.option_value_announcer_chars.contains(&line_char);
                            let has_value = self.can_option_have_value_with_first_char(parse_state, first_char_of_value_is_option_announcer)?;
                            match has_value {
                                OptionHasValueBasedOnFirstChar::Must => {
                                    parse_state.current_option_value_may_be_param = false;
                                    self.begin_parsing_option_value(parse_state, line_char);
                                }
                                OptionHasValueBasedOnFirstChar::Possibly => {
                                    parse_state.current_option_value_may_be_param = true;
                                    self.begin_parsing_option_value(parse_state, line_char);
                                }
                                OptionHasValueBasedOnFirstChar::MustNot => {
                                    parse_state.current_option_value_may_be_param = false;
                                    self.match_option_arg(parse_state, false, args)?; // process current option
                                    parse_state.arg_parse_state = ArgParseState::NotInArg;
                                    more = self.process_char(parse_state, line, line_char, args)? // will handle new option/text param
                                }
                            }
                        }
                    }
                    OptionParseState::InValue => {
                        if let Some(unwrapped_escape_char) = self.escape_char {
                            if unwrapped_escape_char == line_char {
                                parse_state.option_parse_state = OptionParseState::InValueEscaped;
                            }
                        }
        
                        if parse_state.option_parse_state == OptionParseState::InValue {
                            if parse_state.value_quoted {
                                if line_char == parse_state.quote_char {
                                    parse_state.option_parse_state = OptionParseState::InValuePossibleEndQuote;
                                } else {
                                    parse_state.value_bldr.push(line_char);
                                }
                            } else {
                                if !line_char.is_whitespace() {
                                    parse_state.value_bldr.push(line_char);
                                } else {
                                    self.match_option_arg(parse_state, true, args)?;
                                    parse_state.arg_parse_state = ArgParseState::NotInArg;
                                }
                            }
                        }
                    }
                    OptionParseState::InValuePossibleEndQuote => {
                        if line_char == parse_state.quote_char && self.embed_quote_char_with_double {
                            parse_state.value_bldr.push(line_char);
                            parse_state.option_parse_state = OptionParseState::InValue;
                        } else {
                            if line_char.is_whitespace() {
                                self.match_option_arg(parse_state, true, args)?;
                                parse_state.arg_parse_state = ArgParseState::NotInArg;
                            } else {
                                Err(parse_state.create_option_error(ErrorId::QuotedOptionValueNotFollowedByWhitespaceChar))?;
                            }
                        }
                    }
                    OptionParseState::InValueEscaped => {
                        let char_can_be_escaped: bool;
                        if !self.only_escape_quote_char_and_escape_char {
                            char_can_be_escaped = true;
                        } else {
                            if parse_state.quoting_active && line_char == parse_state.quote_char {
                                char_can_be_escaped = true;
                            } else {
                                if let Some(unwrapped_escape_char) = self.escape_char {
                                    if line_char == unwrapped_escape_char {
                                        char_can_be_escaped = true;
                                    } else {
                                        char_can_be_escaped = false;
                                    }
                                } else {
                                    char_can_be_escaped = false;
                                }
                            }
                        }
        
                        if char_can_be_escaped {
                            parse_state.value_bldr.push(line_char);
                            parse_state.option_parse_state = OptionParseState::InValue;
                        }
                    }
                }
            }
        }

        Ok(more)
    }

    fn begin_parsing_option_value(&self, parse_state: &mut ParseState, line_char: char) {
        parse_state.value_bldr.clear();
        parse_state.value_quoted = parse_state.quoting_active && line_char == parse_state.quote_char;
        if !parse_state.value_quoted {
            parse_state.value_bldr.push(line_char);
        }
        parse_state.option_parse_state = OptionParseState::InValue;
    }

    fn create_option_termination_char_array(&self, quoting_active: bool, quote_char: char) -> Vec<char> {
        let mut result = Vec::with_capacity(1 + self.option_value_announcer_chars.len() + self.parse_terminate_chars.len());
        if quoting_active {
            result.push(quote_char);
        }
        result.extend(&self.option_value_announcer_chars);
        result.extend(&self.parse_terminate_chars);

        result
    }

    fn can_option_code_have_value(&self, parse_state: &ParseState) -> bool {
        if self.matchers.is_empty() {
            self.can_option_code_have_value_with_matcher(parse_state, &self.fallback_matcher)
        } else {
            for matcher in &self.matchers {
                if self.can_option_code_have_value_with_matcher(parse_state, matcher) {
                    return true
                }
            }
            false
        }
    }

    fn can_option_code_have_value_with_matcher(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        if self.try_match_option_excluding_value(parse_state, matcher) {
            let option_has_value = matcher.option_has_value.as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            *option_has_value != OptionHasValue::Never
        } else { 
            false
        }
    }

    fn can_option_have_value_with_first_char(&self, parse_state: &ParseState, first_char_of_value_is_option_announcer: bool) -> Result<OptionHasValueBasedOnFirstChar, Error> {
        let mut has_value: OptionHasValueBasedOnFirstChar;
        if self.matchers.is_empty() {
            self.can_option_have_value_with_first_char_with_matcher(parse_state, first_char_of_value_is_option_announcer, &self.fallback_matcher)
        } else {
            has_value = OptionHasValueBasedOnFirstChar::MustNot;
            for matcher in &self.matchers {
                let matched_has_value = self.can_option_have_value_with_first_char_with_matcher(parse_state, first_char_of_value_is_option_announcer, matcher)?;
                match matched_has_value {
                    OptionHasValueBasedOnFirstChar::Must => return Ok(OptionHasValueBasedOnFirstChar::Must),
                    OptionHasValueBasedOnFirstChar::Possibly => has_value = OptionHasValueBasedOnFirstChar::Possibly,
                    OptionHasValueBasedOnFirstChar::MustNot => {},
                }
            }
            Ok(has_value)
        }
    }

    fn can_option_have_value_with_first_char_with_matcher(&self, parse_state: &ParseState,
        first_char_of_value_is_option_announcer: bool,
        matcher: &Matcher<O, P>
    ) -> Result<OptionHasValueBasedOnFirstChar, Error> {
        if self.try_match_option_excluding_value(parse_state, matcher) {
            let option_has_value = matcher.option_has_value.as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            match *option_has_value {
                OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer => {
                    if first_char_of_value_is_option_announcer {
                        Err(parse_state.create_option_error(ErrorId::OptionValueCannotBeginWithOptionAnnouncer))
                    } else {
                        Ok(OptionHasValueBasedOnFirstChar::Must)
                    }
                }
                OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer => {
                    Ok(OptionHasValueBasedOnFirstChar::Must)
                }
                OptionHasValue::IfPossible => {
                    if parse_state.option_value_announcer_is_ambiguous {
                        if first_char_of_value_is_option_announcer {
                            Ok(OptionHasValueBasedOnFirstChar::MustNot)
                        } else {
                            Ok(OptionHasValueBasedOnFirstChar::Possibly)
                        }
                    } else {
                        if first_char_of_value_is_option_announcer {
                            Err(parse_state.create_option_error(ErrorId::OptionValueCannotBeginWithOptionAnnouncer))
                        } else {
                            Ok(OptionHasValueBasedOnFirstChar::Must)
                        }
                    }
                }
                OptionHasValue::Never => {
                    unreachable!("Unexpected never branch in function: \"{}\", module: \"{}\"", "", module_path!());
                }
            }
        } else {
            Ok(OptionHasValueBasedOnFirstChar::MustNot)
        }
    }

    // fn can_option_have_value(&self, session: &ParseState, value_first_char: Option<&char>) -> bool {
    //     let value_first_char_is_option_announcer = self.option_announcer_chars.contains(value_first_char);

    //     if self.matchers.is_empty() {
    //         self.can_option_have_value_with_matcher(session, ambiguous_value_announcer, value_first_char_is_option_announcer, &self.fallback_matcher)
    //     } else {
    //         for matcher in &self.matchers {
    //             if self.can_option_have_value_with_matcher(session, ambiguous_value_announcer, value_first_char_is_option_announcer, matcher) {
    //                 return true
    //             }
    //         }
    //         false
    //     }
    // }

    // fn can_option_have_value_with_matcher(&self,
    //     session: &ParseState,
    //     ambiguous_value_announcer: bool,
    //     value_first_char_is_option_announcer: bool,
    //     matcher: &Matcher<O, P>
    // ) -> bool {
    //     if  self.try_match_index(&session.arg_count, &matcher.arg_indices)
    //         &&
    //         self.try_match_bool(true, matcher.is_option)
    //         &&
    //         self.try_match_index(&session.option_count, &matcher.option_indices)
    //         &&
    //         self.try_match_option_code(&session.option_code, &matcher.option_codes)
    //     {
    //         let option_has_value = matcher.option_has_value.unwrap_or(DEFAULT_OPTION_HAS_VALUE);
    //         if option_has_value == OptionHasValue::Never {
    //             false
    //         } else {
    //             if !value_first_char_is_option_announcer {
    //                 true
    //             } else {
    //                 let matcher_option_value_can_start_with_announcer_char = matcher.option_value_can_start_with_announcer_char.unwrap_or(DEFAULT_OPTION_VALUE_CAN_START_WITH_ANNOUNCER_CHAR);
    //                 if matcher_option_value_can_start_with_announcer_char {
    //                     let matcher_option_has_value = matcher.option_has_value.as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
    //                     *matcher_option_has_value == OptionHasValue::Always
    //                 } else {
    //                     false
    //                 }
    //             }
    //         }
    //     } else {
    //         false
    //     }
    // }

    fn match_option_arg<'a>(&'a self, parse_state: &mut ParseState, has_value: bool, args: &mut Args<'a, O, P>) -> Result<(), Error> {
        let mut optioned_matcher = self.try_find_option_matcher(parse_state, has_value);
        if let Some(matcher) = optioned_matcher {
            self.add_option_arg(parse_state, has_value, matcher, args);
            Ok(())
        } else {
            if has_value && parse_state.current_option_value_may_be_param {
                // Ambiguous value announcer (white space). Value may have been a parameter.
                // Try matching option without value and then add subsequent arg as parameter.
                optioned_matcher = self.try_find_option_matcher(parse_state, false);
                if let Some(matcher) = optioned_matcher {
                    self.add_option_arg(parse_state, false, matcher, args);
                    self.match_param_arg(parse_state, args)?;
                    Ok(())
                } else {
                    Err(parse_state.create_option_error(ErrorId::UnmatchedOption))
                }
            } else {
                Err(parse_state.create_option_error(ErrorId::UnmatchedOption))
            }
        }
    }

    fn add_option_arg<'a>(&self, parse_state: &mut ParseState, has_value: bool, matcher: &'a Matcher<O, P>, args: &mut Args<'a, O, P>) {
        let value_text = if has_value {
            Some(parse_state.value_bldr.clone())
        } else {
            None
        };
        let properties = OptionProperties {
            matcher,
            line_char_index: parse_state.arg_start_line_char_idx,
            arg_index: parse_state.arg_count,
            option_index: parse_state.option_count,
            code: parse_state.option_code.clone(),
            value_text
        };

        let arg = Arg::Option(properties);
        args.push(arg);

        parse_state.arg_count += 1;
        parse_state.option_count += 1;
    }

    fn try_find_option_matcher(&self, parse_state: &ParseState, has_value: bool) -> Option<&Matcher<O, P>> {
        if self.matchers.is_empty() {
            Some(&self.fallback_matcher)
        } else {
            self.matchers.iter().find(|&matcher| self.try_match_option(parse_state, has_value, matcher))
        }
    }

    fn try_match_option(&self, parse_state: &ParseState, has_value: bool, matcher: &Matcher<O, P>) -> bool {
        if  self.try_match_option_excluding_value(parse_state, matcher) {
            // want to match value
            let unwrapped_matcher_option_has_value = matcher.option_has_value.as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            match *unwrapped_matcher_option_has_value {
                OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer => {
                    // matcher expects value
                    if has_value {
                        // option has value - try match
                        self.try_match_value_text(&parse_state.value_bldr, &matcher.value_text, self.option_codes_case_sensitive)
                    } else {
                        // option does not have value
                        false
                    }
                }
                OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer => {
                    // In can_option_have_value_with_first_char_with_matcher() we errored cases where Value starts with Option Announcer, so
                    // treat same as OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer

                    // matcher expects value
                    if has_value {
                        // option has value - try match
                        self.try_match_value_text(&parse_state.value_bldr, &matcher.value_text, self.option_codes_case_sensitive)
                    } else {
                        // option does not have value
                        false
                    }
                }
                OptionHasValue::IfPossible => {
                    // In can_option_have_value_with_first_char_with_matcher() we either errored or disallowed cases where Value starts with
                    // Option Announcer. So can assume that value here does not start with option announcer

                    // matcher specifies that option either can or cannot have value
                    if has_value {
                        // option has value - try match
                        self.try_match_value_text(&parse_state.value_bldr, &matcher.value_text, self.option_codes_case_sensitive)
                    } else {
                        // option does not have value
                        true
                    }
                }
                OptionHasValue::Never => {
                    // option does not expect value - return false if has value
                    !has_value
                }
            }
        } else {
            false
        }
    }

    fn match_param_arg<'a>(&'a self, parse_state: &mut ParseState, args: &mut Args<'a, O, P>) -> Result<(), Error> {
        let optioned_matcher = if self.matchers.is_empty() {
            Some(&self.fallback_matcher)
        } else {
            self.matchers.iter().find(|&matcher| self.try_match_param(parse_state, matcher))
        };

        if let Some(matcher) = optioned_matcher {
            self.add_param_arg(parse_state, matcher, args);
            Ok(())
        } else {
            Err(parse_state.create_param_error(ErrorId::UnmatchedParam))
        }
    }

    fn add_param_arg<'a>(&self, parse_state: &mut ParseState, matcher: &'a Matcher<O, P>, args: &mut Args<'a, O, P>) {
        let properties = ParamProperties {
            matcher,
            line_char_index: parse_state.arg_start_line_char_idx,
            arg_index: parse_state.arg_count,
            param_index: parse_state.param_count,
            value_text: parse_state.value_bldr.clone(),
        };

        let arg = Arg::Param(properties);
        args.push(arg);

        parse_state.arg_count += 1;
        parse_state.param_count += 1;
    }

    fn try_match_option_excluding_value(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, &matcher.arg_indices)
        &&
        self.try_match_arg_type(ArgType::Option, &matcher.arg_type)
        &&
        self.try_match_index(&parse_state.option_count, &matcher.option_indices)
        &&
        self.try_match_option_code(&parse_state.option_code, &matcher.option_codes)
    }

    fn try_match_param(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, &matcher.arg_indices)
        &&
        self.try_match_arg_type(ArgType::Param, &matcher.arg_type)
        &&
        self.try_match_index(&parse_state.param_count, &matcher.param_indices)
        &&
        self.try_match_value_text(&parse_state.value_bldr, &matcher.value_text, self.params_case_sensitive)
    }

    fn try_match_index(&self, index: &usize, matcher_indices: &Option<Vec<usize>>) -> bool {
        if let Some(unwrapped_matcher_indices) = matcher_indices {
            unwrapped_matcher_indices.contains(index)
        } else {
            true
        }
    }

    fn try_match_arg_type(&self, value: ArgType, matcher_value: &Option<ArgType>) -> bool {
        if let Some(unwrapped_matcher_value) = matcher_value {
            *unwrapped_matcher_value == value 
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

    fn try_match_value_text(&self, value_text: &str, matcher_value_text: &Option<RegexOrText>, case_sensitive: bool) -> bool {
        if let Some(matcher_value_text) = matcher_value_text {
            matcher_value_text.is_match(value_text, case_sensitive)
        } else {
            true
        }
    }
}

enum OptionHasValueBasedOnFirstChar {
    Must,
    Possibly,
    MustNot,
}