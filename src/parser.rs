use std::env;
use crate::env_char::EnvChar;
use crate::parse_error_id::{ParseErrorId};
use crate::parse_error::{ParseError};
use crate::regex_or_text::{RegexOrText};
use crate::arg::{Arg, Args, OptionProperties, ParamProperties, BinaryProperties};
use crate::matcher::{Matcher, Matchers, OptionHasValue, DefaultTagType, DEFAULT_OPTION_HAS_VALUE, MatchArgType};
use crate::parse_state::{ParseState, ArgParseState, OptionParseState};

pub const DEFAULT_LINE_QUOTE_CHARS: [char; 1] = ['"'];
pub const DEFAULT_LINE_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
pub const DEFAULT_LINE_OPTION_CODES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_LINE_OPTION_CODE_CAN_BE_EMPTY: bool = false;
pub const DEFAULT_LINE_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
pub const DEFAULT_LINE_OPTION_VALUE_ANNOUNCER_CHARS: [char; 1] = [' '];
pub const DEFAULT_LINE_OPTION_VALUES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_LINE_PARAMS_CASE_SENSITIVE: bool = false;
pub const DEFAULT_LINE_EMBED_QUOTE_CHAR_WITH_DOUBLE: bool = true;
pub const DEFAULT_LINE_ESCAPE_CHAR: Option<char> = None;
pub const DEFAULT_LINE_ESCAPEABLE_LOGICAL_CHARS: [EscapeableLogicalChar; 2] = [EscapeableLogicalChar::Escape, EscapeableLogicalChar::Quote];
pub const DEFAULT_LINE_ESCAPEABLE_CHARS: [char; 0] = [];
pub const DEFAULT_LINE_PARSE_TERMINATE_CHARS: [char; 3] = ['<', '>', '|'];
pub const DEFAULT_LINE_FIRST_ARG_IS_BINARY: bool = true;

pub const DEFAULT_ENV_ARGS_QUOTE_CHARS: [char; 1] = ['"'];
pub const DEFAULT_ENV_ARGS_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
pub const DEFAULT_ENV_ARGS_OPTION_CODES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_ENV_ARGS_OPTION_CODE_CAN_BE_EMPTY: bool = false;
pub const DEFAULT_ENV_ARGS_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
pub const DEFAULT_ENV_ARGS_OPTION_VALUE_ANNOUNCER_CHARS: [char; 1] = [' '];
pub const DEFAULT_ENV_ARGS_OPTION_VALUES_CASE_SENSITIVE: bool = false;
pub const DEFAULT_ENV_ARGS_PARAMS_CASE_SENSITIVE: bool = false;
pub const DEFAULT_ENV_ARGS_EMBED_QUOTE_CHAR_WITH_DOUBLE: bool = true;
pub const DEFAULT_ENV_ARGS_ESCAPE_CHAR: Option<char> = None;
pub const DEFAULT_ENV_ARGS_ESCAPEABLE_LOGICAL_CHARS: [EscapeableLogicalChar; 2] = [EscapeableLogicalChar::Escape, EscapeableLogicalChar::Quote];
pub const DEFAULT_ENV_ARGS_ESCAPEABLE_CHARS: [char; 0] = [];
pub const DEFAULT_ENV_ARGS_PARSE_TERMINATE_CHARS: [char; 3] = ['<', '>', '|'];
pub const DEFAULT_ENV_ARGS_FIRST_ARG_IS_BINARY: bool = true;

pub struct Parser<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    /// The array of characters any of which can be used as a quote character.  A quote character is used to enclose all text in a parameter
    /// or an option value.
    ///
    /// Whitespace characters (normally spaces) are used to delimit arguments in a command line.  If a parameter or an option value contain
    /// whitespace characters, place a quote character at either end of the parameter or value text.  If the parameter or option value already contain 
    /// one or more quote characters, then these can be embedded using either [`Double quote characters`](Parser::embed_quote_char_with_double) or
    /// [`Escaping`](Parser::escape_char)
    ///
    /// If text starts with a quote character, you also need to embed it with either quoting or escaping or enclose it with a different quote character.
    /// You can also use quoting to enclose text which begins with a option announcer but is not an option. See
    /// [`Matcher.option_has_value`](Matcher::option_has_value) for alternative ways of handling text beginning with the option announcer character.
    ///
    /// Default: `"` (Double quote character is the only character in the array)
    quote_chars: Vec<char>,
    /// The array of characters any of which can be used to signify the start of an option argument in the command line.
    ///
    /// Normally a command line argument which begins with one of the characters in this array will be parsed as a option. However this behaviour
    /// can be overridden with [`Matcher.option_has_value`](Matcher::option_has_value).
    ///
    /// Default: `-` (Dash character is the only character in the array)
    option_announcer_chars: Vec<char>,
    option_codes_case_sensitive: bool,
    option_code_can_be_empty: bool,
    multi_char_option_code_requires_double_announcer: bool,
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
    option_value_announcer_chars: Vec<char>,
    option_values_case_sensitive: bool,
    params_case_sensitive: bool,
    embed_quote_char_with_double: bool,
    escape_char: Option<char>,
    escapeable_logical_chars: Vec<EscapeableLogicalChar>,
    escapeable_chars: Vec<char>,
    first_arg_is_binary: bool,
    /// An array of characters which terminate the parsing of arguments in the command line.
    /// 
    /// If any of the characters in this array are encountered outside a quoted value, then that character
    /// and all remaining characters in the command line are ignored.  This can be used to ignore standard input/output
    /// redirection and the end of a command line.
    ///
    /// Default: `<>|`  (standard input redirection to file, standard output redirection to file, pipe standard output)
    parse_terminate_chars: Vec<char>,

    matchers: Matchers<O, P>,
    any_matcher: Matcher<O, P>,
}

impl<O: Default, P: Default> Parser<O, P> {
    pub fn new() -> Self {
        Parser {
            quote_chars: DEFAULT_LINE_QUOTE_CHARS.to_vec(),
            option_announcer_chars: DEFAULT_LINE_OPTION_ANNOUNCER_CHARS.to_vec(),
            option_codes_case_sensitive: DEFAULT_LINE_OPTION_CODES_CASE_SENSITIVE,
            option_code_can_be_empty: DEFAULT_LINE_OPTION_CODE_CAN_BE_EMPTY,
            multi_char_option_code_requires_double_announcer: DEFAULT_LINE_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
            option_value_announcer_chars: DEFAULT_LINE_OPTION_VALUE_ANNOUNCER_CHARS.to_vec(),
            option_values_case_sensitive: DEFAULT_LINE_OPTION_VALUES_CASE_SENSITIVE,
            params_case_sensitive: DEFAULT_LINE_PARAMS_CASE_SENSITIVE,
            embed_quote_char_with_double: DEFAULT_LINE_EMBED_QUOTE_CHAR_WITH_DOUBLE,
            escape_char: DEFAULT_LINE_ESCAPE_CHAR,
            escapeable_logical_chars: DEFAULT_LINE_ESCAPEABLE_LOGICAL_CHARS.to_vec(),
            escapeable_chars: DEFAULT_LINE_ESCAPEABLE_CHARS.to_vec(),
            parse_terminate_chars: DEFAULT_LINE_PARSE_TERMINATE_CHARS.to_vec(),
            first_arg_is_binary: DEFAULT_LINE_FIRST_ARG_IS_BINARY,

            matchers: Matchers::new(),
            any_matcher: Matcher::new(""),
        }
    }

    pub fn new_with_line_defaults() -> Self {
        let mut parser = Parser::new();
        parser.set_line_defaults();
        parser
    }

    pub fn new_with_env_args_defaults() -> Self {
        let mut parser = Parser::new();
        parser.set_env_args_defaults();
        parser
    }

}

impl<O: Default, P: Default> Default for Parser<O, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Default, P: Default> Parser<O, P> {
    pub fn quote_chars(&self) -> &Vec<char> {
        &self.quote_chars
    }

    pub fn set_quote_chars(&mut self, value: &[char]) -> &mut Self {
        self.quote_chars = Vec::from(value);
        self
    }

    pub fn option_announcer_chars(&self) -> &Vec<char> {
        &self.option_announcer_chars
    }

    pub fn set_option_announcer_chars(&mut self, value: &[char]) -> &mut Self {
        self.option_announcer_chars = Vec::from(value);
        self
    }

    pub fn option_codes_case_sensitive(&self) -> bool {
        self.option_codes_case_sensitive
    }

    pub fn set_option_codes_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.option_codes_case_sensitive = value;
        self
    }

    pub fn option_code_can_be_empty(&self) -> bool {
        self.option_code_can_be_empty
    }

    pub fn set_option_code_can_be_empty(&mut self, value: bool) -> &mut Self {
        self.option_code_can_be_empty = value;
        self
    }

    pub fn multi_char_option_code_requires_double_announcer(&self) -> bool {
        self.multi_char_option_code_requires_double_announcer
    }

    pub fn set_multi_char_option_code_requires_double_announcer(&mut self, value: bool) -> &mut Self {
        self.multi_char_option_code_requires_double_announcer = value;
        self
    }

    pub fn option_value_announcer_chars(&self) -> &Vec<char> {
        &self.option_value_announcer_chars
    }

    pub fn set_option_value_announcer_chars(&mut self, value: &[char]) -> &mut Self {
        self.option_value_announcer_chars = Vec::from(value);
        self
    }

    pub fn option_values_case_sensitive(&self) -> bool {
        self.option_values_case_sensitive
    }

    pub fn set_option_values_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.option_values_case_sensitive = value;
        self
    }

    pub fn params_case_sensitive(&self) -> bool {
        self.params_case_sensitive
    }

    pub fn set_params_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.params_case_sensitive = value;
        self
    }

    pub fn embed_quote_char_with_double(&self) -> bool {
        self.embed_quote_char_with_double
    }

    pub fn set_embed_quote_char_with_double(&mut self, value: bool) -> &mut Self {
        self.embed_quote_char_with_double = value;
        self
    }

    pub fn escape_char(&self) -> &Option<char> {
        &self.escape_char
    }

    pub fn set_escape_char(&mut self, value: Option<char>) -> &mut Self {
        self.escape_char = value;
        self
    }

    pub fn some_escape_char(&mut self, value: char) -> &mut Self {
        self.escape_char = Some(value);
        self
    }

    pub fn none_escape_char(&mut self) -> &mut Self {
        self.escape_char = None;
        self
    }

    pub fn escapeable_logical_chars(&self) -> &Vec<EscapeableLogicalChar> {
        &self.escapeable_logical_chars
    }

    pub fn set_escapeable_logical_chars(&mut self, value: &[EscapeableLogicalChar]) -> &mut Self {
        self.escapeable_logical_chars = Vec::from(value);
        self
    }

    pub fn escapeable_chars(&self) -> &Vec<char> {
        &self.escapeable_chars
    }

    pub fn set_escapeable_chars(&mut self, value: &[char]) -> &mut Self {
        self.escapeable_chars = Vec::from(value);
        self
    }

    pub fn first_arg_is_binary(&self) -> bool {
        self.first_arg_is_binary
    }

    pub fn set_first_arg_is_binary(&mut self, value: bool) -> &mut Self {
        self.first_arg_is_binary = value;
        self
    }

    pub fn parse_terminate_chars(&self) -> &Vec<char> {
        &self.parse_terminate_chars
    }

    pub fn set_parse_terminate_chars(&mut self, value: &[char]) -> &mut Self {
        self.parse_terminate_chars = Vec::from(value);
        self
    }

}

impl<O: Default, P: Default> Parser<O, P> {

    pub fn set_line_defaults(&mut self) {
        self
            .set_quote_chars(&DEFAULT_LINE_QUOTE_CHARS)
            .set_option_announcer_chars(&DEFAULT_LINE_OPTION_ANNOUNCER_CHARS)
            .set_option_codes_case_sensitive(DEFAULT_LINE_OPTION_CODES_CASE_SENSITIVE)
            .set_option_code_can_be_empty(DEFAULT_LINE_OPTION_CODE_CAN_BE_EMPTY)
            .set_multi_char_option_code_requires_double_announcer(DEFAULT_LINE_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER)
            .set_option_value_announcer_chars(&DEFAULT_LINE_OPTION_VALUE_ANNOUNCER_CHARS)
            .set_option_values_case_sensitive(DEFAULT_LINE_OPTION_VALUES_CASE_SENSITIVE)
            .set_params_case_sensitive(DEFAULT_LINE_PARAMS_CASE_SENSITIVE)
            .set_embed_quote_char_with_double(DEFAULT_LINE_EMBED_QUOTE_CHAR_WITH_DOUBLE)
            .set_escape_char(DEFAULT_LINE_ESCAPE_CHAR)
            .set_escapeable_logical_chars(&DEFAULT_LINE_ESCAPEABLE_LOGICAL_CHARS)
            .set_escapeable_chars(&DEFAULT_LINE_ESCAPEABLE_CHARS)
            .set_parse_terminate_chars(&DEFAULT_LINE_PARSE_TERMINATE_CHARS)
            .set_first_arg_is_binary(DEFAULT_LINE_FIRST_ARG_IS_BINARY);
    }

    pub fn set_env_args_defaults(&mut self) {
        self
            .set_quote_chars(&DEFAULT_ENV_ARGS_QUOTE_CHARS)
            .set_option_announcer_chars(&DEFAULT_ENV_ARGS_OPTION_ANNOUNCER_CHARS)
            .set_option_codes_case_sensitive(DEFAULT_ENV_ARGS_OPTION_CODES_CASE_SENSITIVE)
            .set_option_code_can_be_empty(DEFAULT_ENV_ARGS_OPTION_CODE_CAN_BE_EMPTY)
            .set_multi_char_option_code_requires_double_announcer(DEFAULT_ENV_ARGS_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER)
            .set_option_value_announcer_chars(&DEFAULT_ENV_ARGS_OPTION_VALUE_ANNOUNCER_CHARS)
            .set_option_values_case_sensitive(DEFAULT_ENV_ARGS_OPTION_VALUES_CASE_SENSITIVE)
            .set_params_case_sensitive(DEFAULT_ENV_ARGS_PARAMS_CASE_SENSITIVE)
            .set_embed_quote_char_with_double(DEFAULT_ENV_ARGS_EMBED_QUOTE_CHAR_WITH_DOUBLE)
            .set_escape_char(DEFAULT_ENV_ARGS_ESCAPE_CHAR)
            .set_escapeable_logical_chars(&DEFAULT_ENV_ARGS_ESCAPEABLE_LOGICAL_CHARS)
            .set_escapeable_chars(&DEFAULT_ENV_ARGS_ESCAPEABLE_CHARS)
            .set_parse_terminate_chars(&DEFAULT_ENV_ARGS_PARSE_TERMINATE_CHARS)
            .set_first_arg_is_binary(DEFAULT_ENV_ARGS_FIRST_ARG_IS_BINARY);
    }

    pub fn get_matchers(&self) -> &Matchers<O, P> {
        &self.matchers
    }

    pub fn push_new_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new(name);
        self.push_matcher(matcher)
    }

    pub fn push_new_option_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new_option(name);
        self.push_matcher(matcher)
    }

    pub fn push_new_param_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new_param(name);
        self.push_matcher(matcher)
    }

    pub fn push_matcher(&mut self, mut matcher: Matcher<O, P>) -> &mut Matcher<O, P> {
        let index = self.matchers.len();
        matcher.set_index(index);
        self.matchers.push(matcher);
        &mut self.matchers[index]
    }

    pub fn delete_matcher(&mut self, name: &str) -> bool {
        if let Some(matcher) = self.find_matcher(name) {
            let idx = matcher.index();
            self.delete_matcher_at(idx);
            true
        } else {
            false
        }
    }

    pub fn delete_matcher_at(&mut self, index: usize) {
        self.matchers.remove(index);
    }

    pub fn clear_matchers(&mut self) {
        self.matchers.clear();
    }

    pub fn find_matcher(&self, name: &str) -> Option<&Matcher<O, P>> {
        self.matchers.iter().find(|&matcher| matcher.name() == name)
    }

    pub fn parse_env(&self) -> Result<Args<O, P>, ParseError> {
        self.parse_env_args(env::args())
    }

    pub fn parse_env_args(&self, env_args: env::Args) -> Result<Args<O, P>, ParseError> {
        let mut args = Vec::new();
        let mut parse_state = ParseState::new(
            "",
            self.first_arg_is_binary,
            self.multi_char_option_code_requires_double_announcer,
        );

        let mut more = true;
        for (env_arg_idx, env_arg) in env_args.enumerate() {
            if env_arg_idx > 0 {
                more = self.process_char(&mut parse_state, EnvChar::Separator, &mut args)?;

                if more {
                    parse_state.increment_line_char_idx();
                }
            } else {
                for unicode_char in env_arg.chars() {
                    let env_char = EnvChar::Unicode(unicode_char);
                    more = self.process_char(&mut parse_state, env_char, &mut args)?;

                    if more {
                        parse_state.increment_line_char_idx();
                    } else {
                        // ignore rest of line
                        break;
                    }
                }
            }

            if !more {
                // ignore rest of line
                break;
            }
        }

        self.finalise_parse(&mut parse_state, &mut args)?;

        Ok(args)
    }

    pub fn parse_line(&self, line: &str) -> Result<Args<O, P>, ParseError> {
        let mut args = Vec::new();

        let mut parse_state = ParseState::new(
            line,
            self.first_arg_is_binary,
            self.multi_char_option_code_requires_double_announcer,
        );

        for char in line.chars() {
            let env_char = EnvChar::Unicode(char);
            let more = self.process_char(&mut parse_state, env_char, &mut args)?;

            if more {
                parse_state.increment_line_char_idx();
            } else {
                // ignore rest of line
                break;
            }
        }

        self.finalise_parse(&mut parse_state, &mut args)?;

        Ok(args)
    }

    fn process_char<'a>(&'a self, parse_state: &mut ParseState, env_char: EnvChar, args: &mut Args<'a, O, P>) -> Result<bool, ParseError> {
        match parse_state.arg_parse_state {
            ArgParseState::WaitBinary => {
                if let Some(unicode_char) = env_char.try_get_unicode_non_whitespace() {
                    if self.parse_terminate_chars.contains(&unicode_char) {
                        Ok(false)
                    } else {
                        self.initialise_param_parsing(parse_state, unicode_char, true);
                        parse_state.arg_parse_state = ArgParseState::InParam;
                        Ok(true)
                    }
                } else {
                    Ok(true)
                }
            }

            ArgParseState::WaitOptionOrParam => {
                if let Some(unicode_char) = env_char.try_get_unicode_non_whitespace() {
                    if self.parse_terminate_chars.contains(&unicode_char) {
                        Ok(false)
                    } else {
                        if self.option_announcer_chars.contains(&unicode_char) {
                            parse_state.arg_parse_state = ArgParseState::InOption;
                            parse_state.option_parse_state = OptionParseState::InCode;
                            parse_state.option_announcer_char = unicode_char;
                            parse_state.arg_start_line_char_idx = parse_state.line_char_idx;
                            parse_state.option_code_start_line_char_idx = parse_state.line_char_idx + 1;
                        } else {
                            parse_state.arg_parse_state = ArgParseState::InParam;
                            self.initialise_param_parsing(parse_state, unicode_char, false);
                        }
                        Ok(true)
                    }
                } else {
                    Ok(true)
                }
            }

            ArgParseState::InParam => {
                match env_char {
                    EnvChar::Separator => {
                        self.match_param_arg(parse_state, args)?;
                        parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                    },
                    EnvChar::Unicode(unicode_char) => {
                        if let Some(unwrapped_escape_char) = self.escape_char {
                            if unwrapped_escape_char == unicode_char {
                                parse_state.arg_parse_state = ArgParseState::InParamEscaped;
                            }
                        }
        
                        if parse_state.arg_parse_state == ArgParseState::InParam {
                            if parse_state.value_quoted {
                                if unicode_char == parse_state.arg_quote_char {
                                    parse_state.arg_parse_state = ArgParseState::InParamPossibleEndQuote;
                                } else {
                                    parse_state.value_bldr.push(unicode_char);
                                }
                            } else {
                                if !unicode_char.is_whitespace() {
                                    parse_state.value_bldr.push(unicode_char);
                                } else {
                                    self.match_param_arg(parse_state, args)?;
                                    parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                                }
                            }
                        }
                    }
                }
                Ok(true)
            }

            ArgParseState::InParamPossibleEndQuote => {
                match env_char {
                    EnvChar::Separator => {
                        self.match_param_arg(parse_state, args)?;
                        parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                    }
                    EnvChar::Unicode(unicode_char) => {
                        if unicode_char == parse_state.arg_quote_char && self.embed_quote_char_with_double {
                            parse_state.value_bldr.push(unicode_char);
                            parse_state.arg_parse_state = ArgParseState::InParam;
                        } else {
                            if unicode_char.is_whitespace() {
                                self.match_param_arg(parse_state, args)?;
                                parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                            } else {
                                Err(parse_state.create_param_error(ParseErrorId::QuotedParamNotFollowedByWhitespaceChar))?;
                            }
                        }
                    }
                }
                Ok(true)
            }

            ArgParseState::InParamEscaped => {
                match env_char {
                    EnvChar::Separator => {
                        Err(parse_state.create_param_error(ParseErrorId::EscapeCharacterAtEndOfParam))?;
                    }
                    EnvChar::Unicode(unicode_char) => {
                        if self.can_char_be_escaped(parse_state, unicode_char) {
                            parse_state.value_bldr.push(unicode_char);
                            parse_state.arg_parse_state = ArgParseState::InParam;
                        } else {
                            Err(parse_state.create_param_error(ParseErrorId::EscapedCharacterInParamCannotBeEscaped))?;
                        }
                    }
                }
                Ok(true)
            }

            ArgParseState::InOption => {
                match parse_state.option_parse_state {
                    OptionParseState::InCode => {
                        match env_char {
                            EnvChar::Separator => {
                                self.finalise_option_code(parse_state, ValueAnnounced::Ambiguous, args)?;
                                Ok(true)
                            },
                            EnvChar::Unicode(unicode_char) => {
                                if self.parse_terminate_chars.contains(&unicode_char) {
                                    self.finalise_option_code(parse_state, ValueAnnounced::Not, args)?;
                                    Ok(false)
                                } else {
                                    if self.option_value_announcer_chars.contains(&unicode_char) {
                                        let value_announced = if unicode_char.is_whitespace() {ValueAnnounced::Ambiguous} else {ValueAnnounced::Definitely};
                                        self.finalise_option_code(parse_state, value_announced, args)?;
                                        Ok(true)
                                    } else {
                                        if self.quote_chars.contains(&unicode_char) {
                                            Err(parse_state.create_option_error(ParseErrorId::OptionCodeCannotContainQuoteChar))
                                        } else {
                                            if let Some(escape_char) = self.escape_char {
                                                if escape_char == unicode_char {
                                                    Err(parse_state.create_option_error(ParseErrorId::OptionCodeCannotContainEscapeChar))
                                                } else {
                                                    Ok(true)
                                                }
                                            } else {
                                                Ok(true)
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                    OptionParseState::WaitOptionValue => {
                        let mut more = true;
                        if let Some(unicode_char) = env_char.try_get_unicode_non_whitespace() {
                            let first_char_of_value_is_option_announcer = self.option_value_announcer_chars.contains(&unicode_char);
                            let has_value = self.can_option_have_value_with_first_char(parse_state, first_char_of_value_is_option_announcer)?;
                            match has_value {
                                OptionHasValueBasedOnFirstChar::Must => {
                                    parse_state.option_parse_state = OptionParseState::InValue;
                                    self.initialise_option_value_parsing(parse_state, unicode_char);
                                    parse_state.current_option_value_may_be_param = false;
                                }
                                OptionHasValueBasedOnFirstChar::Possibly => {
                                    parse_state.option_parse_state = OptionParseState::InValue;
                                    self.initialise_option_value_parsing(parse_state, unicode_char);
                                    parse_state.current_option_value_may_be_param = true;
                                }
                                OptionHasValueBasedOnFirstChar::MustNot => {
                                    parse_state.current_option_value_may_be_param = false;
                                    self.match_option_arg(parse_state, false, args)?; // process current option
                                    parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                                    more = self.process_char(parse_state, env_char, args)? // will handle new option/text param
                                }
                            }
                        }
                        Ok(more)
                    }
                    OptionParseState::InValue => {
                        match env_char {
                            EnvChar::Separator => {
                                self.match_option_arg(parse_state, true, args)?;
                                parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                            },
                            EnvChar::Unicode(unicode_char) => {
                                if let Some(unwrapped_escape_char) = self.escape_char {
                                    if unwrapped_escape_char == unicode_char {
                                        parse_state.option_parse_state = OptionParseState::InValueEscaped;
                                    }
                                }
                
                                if parse_state.option_parse_state == OptionParseState::InValue {
                                    if parse_state.value_quoted {
                                        if unicode_char == parse_state.arg_quote_char {
                                            parse_state.option_parse_state = OptionParseState::InValuePossibleEndQuote;
                                        } else {
                                            parse_state.value_bldr.push(unicode_char);
                                        }
                                    } else {
                                        if !unicode_char.is_whitespace() {
                                            parse_state.value_bldr.push(unicode_char);
                                        } else {
                                            self.match_option_arg(parse_state, true, args)?;
                                            parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                                        }
                                    }
                                }
                            },
                        }
                        Ok(true)
                    }
                    OptionParseState::InValuePossibleEndQuote => {
                        match env_char {
                            EnvChar::Separator => {
                                self.match_option_arg(parse_state, true, args)?;
                                parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                            },
                            EnvChar::Unicode(unicode_char) => {
                                if unicode_char == parse_state.arg_quote_char && self.embed_quote_char_with_double {
                                    parse_state.value_bldr.push(unicode_char);
                                    parse_state.option_parse_state = OptionParseState::InValue;
                                } else {
                                    if unicode_char.is_whitespace() {
                                        self.match_option_arg(parse_state, true, args)?;
                                        parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                                    } else {
                                        Err(parse_state.create_option_error(ParseErrorId::QuotedOptionValueNotFollowedByWhitespaceChar))?;
                                    }
                                }
                            },
                        }
                        Ok(true)
                    }
                    OptionParseState::InValueEscaped => {
                        match env_char {
                            EnvChar::Separator => {
                                Err(parse_state.create_param_error(ParseErrorId::EscapeCharacterAtEndOfOptionValue))?;
                            }
                            EnvChar::Unicode(unicode_char) => {
                                if self.can_char_be_escaped(parse_state, unicode_char) {
                                    parse_state.value_bldr.push(unicode_char);
                                    parse_state.option_parse_state = OptionParseState::InValue;
                                } else {
                                    Err(parse_state.create_param_error(ParseErrorId::EscapedCharacterInOptionValueCannotBeEscaped))?;
                                }
                            }
                        }
                        Ok(true)
                    }
                }
            }
        }
    }

    fn finalise_option_code<'a>(&'a self, parse_state: &mut ParseState, value_announced: ValueAnnounced, args: &mut Args<'a, O, P>)  -> Result<(), ParseError> {
        parse_state.set_option_code(Some(parse_state.line_char_idx))?;
        match value_announced {
            ValueAnnounced::Definitely => {
                if self.can_option_code_have_value(parse_state) {
                    parse_state.option_parse_state = OptionParseState::WaitOptionValue;
                    Ok(())
                } else {
                    Err(parse_state.create_option_error(ParseErrorId::NoMatchForOptionWithValue))
                }
            },
            ValueAnnounced::Ambiguous => {
                if self.can_option_code_have_value(parse_state) {
                    parse_state.option_parse_state = OptionParseState::WaitOptionValue;
                } else {
                    parse_state.current_option_value_may_be_param = false;
                    self.match_option_arg(parse_state, false, args)?;
                    parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                }
                Ok(())
            },
            ValueAnnounced::Not => {
                parse_state.current_option_value_may_be_param = false;
                self.match_option_arg(parse_state, false, args)?;
                parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                Ok(())
            }
        }
    }

    fn finalise_parse<'a>(&'a self, parse_state: &mut ParseState, args: &mut Args<'a, O, P>) -> Result<(), ParseError> {
        match parse_state.arg_parse_state {
            ArgParseState::WaitBinary => {
                Ok(())
            }

            ArgParseState::WaitOptionOrParam => {
                Ok(())
            }

            ArgParseState::InParam => {
                if parse_state.value_quoted {
                    Err(parse_state.create_param_error(ParseErrorId::ParamMissingClosingQuoteCharacter))
                } else {
                    self.match_param_arg(parse_state, args)
                }
            }

            ArgParseState::InParamPossibleEndQuote => {
                self.match_param_arg(parse_state, args)
            }

            ArgParseState::InParamEscaped => {
                Err(parse_state.create_param_error(ParseErrorId::EscapedCharacterInParamCannotBeEscaped))
            }

            ArgParseState::InOption => {
                match parse_state.option_parse_state {
                    OptionParseState::InCode => {
                        parse_state.set_option_code(None)?;
                        parse_state.current_option_value_may_be_param = false;
                        self.match_option_arg(parse_state, false, args)
                    }
                    OptionParseState::WaitOptionValue => {
                        let has_value = self.can_option_have_value_with_first_char(parse_state, false)?;
                        match has_value {
                            OptionHasValueBasedOnFirstChar::Must => {
                                Err(parse_state.create_option_error(ParseErrorId::OptionMissingValue))
                            }
                            OptionHasValueBasedOnFirstChar::Possibly => {
                                parse_state.current_option_value_may_be_param = false;
                                self.match_option_arg(parse_state, false, args)
                            }
                            OptionHasValueBasedOnFirstChar::MustNot => {
                                parse_state.current_option_value_may_be_param = false;
                                self.match_option_arg(parse_state, false, args)
                            }
                        }
                    }
                    OptionParseState::InValue => {
                        if parse_state.value_quoted {
                            Err(parse_state.create_option_error(ParseErrorId::OptionValueMissingClosingQuoteCharacter))
                        } else {
                            self.match_option_arg(parse_state, true, args)
                        }
                    }
                    OptionParseState::InValuePossibleEndQuote => {
                        self.match_option_arg(parse_state, true, args)
                    }
                    OptionParseState::InValueEscaped => {
                        Err(parse_state.create_option_error(ParseErrorId::EscapedCharacterInOptionValueCannotBeEscaped))
                    }
                }
            }
        }
    }

    fn can_char_be_escaped(&self, parse_state: &mut ParseState, unicode_char: char) -> bool {
        for escapeable_logical_char in &self.escapeable_logical_chars {
            match escapeable_logical_char {
                EscapeableLogicalChar::Escape => {
                    if let Some(escapeable_char) = self.escape_char {
                        if escapeable_char == unicode_char {
                            return true;
                        }
                    }
                }
                EscapeableLogicalChar::Quote => {
                    if parse_state.value_quoted && unicode_char == parse_state.arg_quote_char {
                        return true;
                    }
                }
                EscapeableLogicalChar::Whitespace => {
                    if unicode_char.is_whitespace() {
                        return true;
                    }
                }
                EscapeableLogicalChar::OptionAnnouncer => {
                    if self.option_announcer_chars.contains(&unicode_char) {
                        return true;
                    }
                }
                EscapeableLogicalChar::OptionValueAnnouncer => {
                    if self.option_value_announcer_chars.contains(&unicode_char) {
                        return true;
                    }
                }
                EscapeableLogicalChar::All => {
                    return true;
                }
            
            }
        }
        for escapeable_char in &self.escapeable_chars {
            if *escapeable_char == unicode_char {
                return true;
            }
        }
        false
    }

    fn initialise_param_parsing(&self, parse_state: &mut ParseState, unicode_char: char, is_binary: bool) {
        parse_state.value_bldr.clear();
        parse_state.arg_start_line_char_idx = parse_state.line_char_idx;
        parse_state.value_quoted = self.quote_chars.contains(&unicode_char);
        if parse_state.value_quoted {
            parse_state.arg_quote_char = unicode_char;
        } else {
            parse_state.value_bldr.push(unicode_char);
        }
        parse_state.current_param_is_binary = is_binary;
    }

    fn initialise_option_value_parsing(&self, parse_state: &mut ParseState, unicode_char: char) {
        parse_state.value_bldr.clear();
        parse_state.value_quoted = self.quote_chars.contains(&unicode_char);
        if parse_state.value_quoted {
            parse_state.arg_quote_char = unicode_char;
        } else {
            parse_state.value_bldr.push(unicode_char);
        }
    }

    fn can_option_code_have_value(&self, parse_state: &ParseState) -> bool {
        if self.matchers.is_empty() {
            self.can_option_code_have_value_with_matcher(parse_state, &self.any_matcher)
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
            let option_has_value = matcher.option_has_value().as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            *option_has_value != OptionHasValue::Never
        } else { 
            false
        }
    }

    fn can_option_have_value_with_first_char(&self, parse_state: &ParseState, first_char_of_value_is_option_announcer: bool) -> Result<OptionHasValueBasedOnFirstChar, ParseError> {
        let mut has_value: OptionHasValueBasedOnFirstChar;
        if self.matchers.is_empty() {
            self.can_option_have_value_with_first_char_with_matcher(parse_state, first_char_of_value_is_option_announcer, &self.any_matcher)
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
    ) -> Result<OptionHasValueBasedOnFirstChar, ParseError> {
        if self.try_match_option_excluding_value(parse_state, matcher) {
            let option_has_value = matcher.option_has_value().as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            match *option_has_value {
                OptionHasValue::Always => {
                    if matcher.option_value_can_start_with_option_announcer() {
                        Ok(OptionHasValueBasedOnFirstChar::Must)
                    } else {
                        if first_char_of_value_is_option_announcer {
                            Err(parse_state.create_option_error(ParseErrorId::OptionValueCannotBeginWithOptionAnnouncer))
                        } else {
                            Ok(OptionHasValueBasedOnFirstChar::Must)
                        }
                    }
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
                            Err(parse_state.create_option_error(ParseErrorId::OptionValueCannotBeginWithOptionAnnouncer))
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

    fn match_option_arg<'a>(&'a self, parse_state: &mut ParseState, has_value: bool, args: &mut Args<'a, O, P>) -> Result<(), ParseError> {
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
                    Err(parse_state.create_option_error(ParseErrorId::UnmatchedOption))
                }
            } else {
                Err(parse_state.create_option_error(ParseErrorId::UnmatchedOption))
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
            Some(&self.any_matcher)
        } else {
            self.matchers.iter().find(|&matcher| self.try_match_option(parse_state, has_value, matcher))
        }
    }

    fn try_match_option(&self, parse_state: &ParseState, has_value: bool, matcher: &Matcher<O, P>) -> bool {
        if  self.try_match_option_excluding_value(parse_state, matcher) {
            // want to match value
            let unwrapped_matcher_option_has_value = matcher.option_has_value().as_ref().unwrap_or(&DEFAULT_OPTION_HAS_VALUE);
            match *unwrapped_matcher_option_has_value {
                OptionHasValue::Always => {
                    // matcher expects value
                    //
                    // In can_option_have_value_with_first_char_with_matcher() we errored cases where Value starts with Option Announcer, so
                    // can ignore matcher.option_value_can_start_with_option_announcer here.
                    if has_value {
                        // option has value - try match
                        self.try_match_value_text(&parse_state.value_bldr, matcher.value_text(), self.option_codes_case_sensitive)
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
                        self.try_match_value_text(&parse_state.value_bldr, matcher.value_text(), self.option_codes_case_sensitive)
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

    fn match_param_arg<'a>(&'a self, parse_state: &mut ParseState, args: &mut Args<'a, O, P>) -> Result<(), ParseError> {
        if parse_state.current_param_is_binary {
            self.match_binary_arg(parse_state, args)
        } else {
            let optioned_matcher = if self.matchers.is_empty() {
                Some(&self.any_matcher)
            } else {
                self.matchers.iter().find(|&matcher| self.try_match_param(parse_state, matcher))
            };

            if let Some(matcher) = optioned_matcher {
                self.add_param_arg(parse_state, matcher, args);
                Ok(())
            } else {
                Err(parse_state.create_param_error(ParseErrorId::UnmatchedParam))
            }
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

    fn match_binary_arg<'a>(&'a self, parse_state: &mut ParseState, args: &mut Args<'a, O, P>) -> Result<(), ParseError> {
        self.add_binary_arg(parse_state, &self.any_matcher, args);
        Ok(())
    }

    fn add_binary_arg<'a>(&self, parse_state: &mut ParseState, matcher: &'a Matcher<O, P>, args: &mut Args<'a, O, P>) {
        let properties = BinaryProperties {
            matcher,
            line_char_index: parse_state.arg_start_line_char_idx,
            arg_index: parse_state.arg_count,
            value_text: parse_state.value_bldr.clone(),
        };

        let arg = Arg::Binary(properties);
        args.push(arg);

        parse_state.arg_count += 1;
    }

    fn try_match_option_excluding_value(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, matcher.arg_indices())
        &&
        self.try_match_arg_type(MatchArgType::Option, matcher.arg_type())
        &&
        self.try_match_index(&parse_state.option_count, matcher.option_indices())
        &&
        self.try_match_option_code(&parse_state.option_code, matcher.option_codes())
    }

    fn try_match_param(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, matcher.arg_indices())
        &&
        self.try_match_arg_type(MatchArgType::Param, matcher.arg_type())
        &&
        self.try_match_index(&parse_state.param_count, matcher.param_indices())
        &&
        self.try_match_value_text(&parse_state.value_bldr, matcher.value_text(), self.params_case_sensitive)
    }

    fn try_match_index(&self, index: &usize, matcher_indices: &Option<Vec<usize>>) -> bool {
        if let Some(unwrapped_matcher_indices) = matcher_indices {
            unwrapped_matcher_indices.contains(index)
        } else {
            true
        }
    }

    fn try_match_arg_type(&self, value: MatchArgType, matcher_value: &Option<MatchArgType>) -> bool {
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

#[derive(Clone)]
pub enum EscapeableLogicalChar {
    Escape,
    Quote,
    Whitespace,
    OptionAnnouncer,
    OptionValueAnnouncer,
    All,
}

enum ValueAnnounced {
    Definitely,
    Ambiguous,
    Not,
}

enum OptionHasValueBasedOnFirstChar {
    Must,
    Possibly,
    MustNot,
}