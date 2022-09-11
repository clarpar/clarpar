use std::env;
use crate::env_char::EnvChar;
use crate::parse_error_type_id::{ParseErrorTypeId};
use crate::parse_error::{ParseError};
use crate::regex_or_text::{RegexOrText};
use crate::arg::{Arg, Args, OptionProperties, ParamProperties, BinaryProperties};
use crate::matcher::{Matcher, Matchers, OptionHasValue, DefaultTagType, MatchArgTypeId};
use crate::parse_state::{ParseState, ArgParseState, OptionParseState};

/// Default [quote characters](Parser::quote_chars) for line parsing.
pub const DEFAULT_LINE_QUOTE_CHARS: [char; 1] = ['"'];
/// Default [option announcer characters](Parser::option_announcer_chars) for line parsing.
pub const DEFAULT_LINE_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
/// Default [option codes case sensitive](Parser::option_codes_case_sensitive) for line parsing.
pub const DEFAULT_LINE_OPTION_CODES_CASE_SENSITIVE: bool = false;
/// Default [option code can be empty](Parser::option_code_can_be_empty) for line parsing.
pub const DEFAULT_LINE_OPTION_CODE_CAN_BE_EMPTY: bool = false;
/// Default [multi character option code requires double announcer](Parser::multi_char_option_code_requires_double_announcer) for line parsing.
pub const DEFAULT_LINE_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
/// Default [option value announcer characters](Parser::option_value_announcer_chars) for line parsing.
pub const DEFAULT_LINE_OPTION_VALUE_ANNOUNCER_CHARS: [char; 1] = [' '];
/// Default [option values case sensitive](Parser::option_values_case_sensitive) for line parsing.
pub const DEFAULT_LINE_OPTION_VALUES_CASE_SENSITIVE: bool = false;
/// Default [parameters case sensitive](Parser::params_case_sensitive) for line parsing.
pub const DEFAULT_LINE_PARAMS_CASE_SENSITIVE: bool = false;
/// Default [embed quote character with double](Parser::embed_quote_char_with_double) for line parsing.
pub const DEFAULT_LINE_EMBED_QUOTE_CHAR_WITH_DOUBLE: bool = true;
/// Default [escape character](Parser::escape_char) for line parsing.
pub const DEFAULT_LINE_ESCAPE_CHAR: Option<char> = None;
/// Default [escapable logical characters](Parser::escapeable_logical_chars) for line parsing.
pub const DEFAULT_LINE_ESCAPEABLE_LOGICAL_CHARS: [EscapeableLogicalChar; 2] = [EscapeableLogicalChar::Escape, EscapeableLogicalChar::Quote];
/// Default [escapable characters](Parser::escapeable_chars) for line parsing.
pub const DEFAULT_LINE_ESCAPEABLE_CHARS: [char; 0] = [];
/// Default [parse terminate characters](Parser::parse_terminate_chars) for line parsing.
pub const DEFAULT_LINE_PARSE_TERMINATE_CHARS: [char; 0] = [];
/// Default [first argument is binary](Parser::first_arg_is_binary) for line parsing.
pub const DEFAULT_LINE_FIRST_ARG_IS_BINARY: bool = true;

/// Default [quote characters](Parser::quote_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_QUOTE_CHARS: [char; 0] = [];
/// Default [option announcer characters](Parser::option_announcer_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_OPTION_ANNOUNCER_CHARS: [char; 1] = ['-'];
/// Default [option codes case sensitive](Parser::option_codes_case_sensitive) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_OPTION_CODES_CASE_SENSITIVE: bool = false;
/// Default [option code can be empty](Parser::option_code_can_be_empty) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_OPTION_CODE_CAN_BE_EMPTY: bool = false;
/// Default [multi character option code requires double announcer](Parser::multi_char_option_code_requires_double_announcer)
/// for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER: bool = false;
/// Default [option value announcer characters](Parser::option_value_announcer_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_OPTION_VALUE_ANNOUNCER_CHARS: [char; 1] = [' '];
/// Default [option values case sensitive](Parser::option_values_case_sensitive) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_OPTION_VALUES_CASE_SENSITIVE: bool = false;
/// Default [parameters case sensitive](Parser::params_case_sensitive) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_PARAMS_CASE_SENSITIVE: bool = false;
/// Default [embed quote character with double](Parser::embed_quote_char_with_double) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_EMBED_QUOTE_CHAR_WITH_DOUBLE: bool = false;
/// Default [escape character](Parser::escape_char) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_ESCAPE_CHAR: Option<char> = None;
/// Default [escapable logical characters](Parser::escapeable_logical_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_ESCAPEABLE_LOGICAL_CHARS: [EscapeableLogicalChar; 2] = [EscapeableLogicalChar::Escape, EscapeableLogicalChar::Quote];
/// Default [escapable characters](Parser::escapeable_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_ESCAPEABLE_CHARS: [char; 0] = [];
/// Default [parse terminate characters](Parser::parse_terminate_chars) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_PARSE_TERMINATE_CHARS: [char; 0] = [];
/// Default [first argument is binary](Parser::first_arg_is_binary) for environment arguments parsing.
pub const DEFAULT_ENV_ARGS_FIRST_ARG_IS_BINARY: bool = true;

/// A Parser is used to parse a command line (or environmental arguments).  It has:
/// * properties which define the style of the command line to be parsed,
/// * a matchers list which specifies the kind of arguments in the command line,
/// * parse functions which parse a command line (or environmental arguments) and return a vector of the parsed arguments.
/// 
/// To use the parser, configure it with the style of the command line, add matchers to cover all possible arguments that a command
/// line can have and then use a parse function to parse a command line (or environmental arguments).
/// 
/// The style of a command line can configured with the following:
/// * Parameters and option values can be quoted ([quote_chars](Self::quote_chars))
/// * Allow quote characters to be embedded in quoted parameters and option values using double quotes
/// ([embed_quote_char_with_double](Self::embed_quote_char_with_double))
/// * Whether parameters are case sensitive ([params_case_sensitive](Self::params_case_sensitive))
/// * The characters which announce an option ([option_announcer_chars](Self::option_announcer_chars))
/// * Whether option codes with more than one character require 2 announcer characters
/// ([multi_char_option_code_requires_double_announcer](Self::multi_char_option_code_requires_double_announcer))
/// * Whether option codes are case sensitive ([option_codes_case_sensitive](Self::option_codes_case_sensitive))
/// * Whether option codes can be empty strings ([option_code_can_be_empty](Self::option_code_can_be_empty))
/// * The characters which announce an option value ([option_value_announcer_chars](Self::option_value_announcer_chars))
/// * Whether option values are case sensitive ([option_values_case_sensitive](Self::option_values_case_sensitive))
/// * Optionally define a character which will escape characters with special purpose ([escape_char](Self::escape_char))
/// * Specify which which special purpose characters can be escaped ([escapeable_logical_chars](Self::escapeable_logical_chars))
/// * Specify which literal characters can be escaped ([escapeable_chars](Self::escapeable_chars))
/// * Whether the first argument is the binary name ([first_arg_is_binary](Self::first_arg_is_binary))
/// * The characters which will terminate the parsing of the line early ([parse_terminate_chars](Self::parse_terminate_chars))
///
/// The [new](Self::new) constructor will create a Parser with base defaults that are a good starting point for parsing a command line.
/// The [with_env_args_defaults](Self::with_env_args_defaults) constructor has base defaults for parsing environmental arguments.
/// These defaults can also be applied with the [set_line_defaults](Self::set_line_defaults) and
/// [set_env_args_defaults](Self::set_env_args_defaults) functions.
/// 
/// The following functions can be used to manage the matcher list: [matchers](Self::matchers), [push_new_matcher](Self::push_new_matcher)
/// [push_new_option_matcher](Self::push_new_option_matcher), [push_new_param_matcher](Self::push_new_param_matcher),
/// [push_matcher](Self::push_matcher), [delete_matcher_at](Self::delete_matcher_at), [clear_matchers](Self::clear_matchers) and
/// [find_matcher](Self::find_matcher).
/// 
/// There are 3 functions for parsing a command line or environmental variables:
/// * [parse_line](Self::parse_line) - Parses a command line
/// * [parse_env_args](Self::parse_env_args) - Parses an environmental variables specified in a `std::env::args` iterator
/// * [parse_env](Self::parse_env) - Parses the application's environmental variables
/// 
/// If parsing was successful, these 3 functions will return a vector of [parsed arguments](Args). Otherwise they will return an
/// [ParseError](ParseError) struct detailing the type of parse error and its location in the line.
pub struct Parser<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    quote_chars: Vec<char>,
    option_announcer_chars: Vec<char>,
    option_codes_case_sensitive: bool,
    option_code_can_be_empty: bool,
    multi_char_option_code_requires_double_announcer: bool,
    option_value_announcer_chars: Vec<char>,
    option_values_case_sensitive: bool,
    params_case_sensitive: bool,
    embed_quote_char_with_double: bool,
    escape_char: Option<char>,
    escapeable_logical_chars: Vec<EscapeableLogicalChar>,
    escapeable_chars: Vec<char>,
    first_arg_is_binary: bool,
    parse_terminate_chars: Vec<char>,

    matchers: Matchers<O, P>,
    any_matcher: Matcher<O, P>,
}

impl<O: Default, P: Default> Parser<O, P> {
    /// Create a new Parser object with Line parsing defaults. The `O` and `P` generic parameters specify the types that
    /// can be used to tag matchers with a value which easily enables Option and Parameter arguments to be identified.
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

    /// Create a new Parser object with Line parsing defaults. The `O` and `P` generic parameters specify the types that
    /// can be used to tag matchers with a value which easily enables Option and Parameter arguments to be identified.
    pub fn with_line_defaults() -> Self {
        let mut parser = Parser::new();
        parser.set_line_defaults();
        parser
    }

    /// Create a new Parser object with Environment Arguments parsing defaults. The `O` and `P` generic parameters specify the types that
    /// can be used to tag matchers with a value which easily enables Option and Parameter arguments to be identified.
    pub fn with_env_args_defaults() -> Self {
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
    /// Get the array of characters any of which can be used as a quote character.  A quote character is used to enclose all text in a parameter
    /// or an option value.
    ///
    /// Whitespace characters (normally spaces) are used to delimit arguments in a command line.  If a parameter or an option value contain
    /// whitespace characters, place a quote character at either end of the parameter or value text.  If the parameter or option value already contain 
    /// one or more quote characters, then these can be embedded using either [`Double quote characters`](Parser::embed_quote_char_with_double) or
    /// [`Escaping`](Parser::escape_char)
    ///
    /// If text starts with a quote character, you also need to embed it with either quoting or escaping or enclose it with a different quote character.
    /// You can also use quoting to enclose text which begins with a option announcer but is not an option. See
    /// [`Matcher::option_has_value`](Matcher::option_has_value) for alternative ways of handling text beginning with the option announcer character.
    ///
    /// Line Default: `['"']` (Array with one double quote character)\
    /// Env args Default: `[]` (Empty array)
    pub fn quote_chars(&self) -> &[char] {
        &self.quote_chars
    }

    /// Get the array of characters any of which can be used as a quote character. See [Parser::quote_chars](Parser::quote_chars) for more details.
    pub fn set_quote_chars(&mut self, value: &[char]) -> &mut Self {
        self.quote_chars = Vec::from(value);
        self
    }

    /// Get the array of characters any of which can be used to signify the start of an option argument in the command line.
    ///
    /// A command line argument which begins with one of the characters in this array will be parsed as a option.
    /// 
    /// Note that the Parser can be configured to allow option values to begin with an option announcer character in some circumstances.
    /// See [Matcher.option_value_can_start_with_option_announcer](Matcher::option_value_can_start_with_option_announcer) for more details.
    ///
    /// Default: `['-']` (Array with one dash character)
    pub fn option_announcer_chars(&self) -> &[char] {
        &self.option_announcer_chars
    }

    /// Set the array of characters any of which can be used to signify the start of an option argument in the command line. See
    /// [Parser::option_announcer_chars](Parser::option_announcer_chars) for more details.
    pub fn set_option_announcer_chars(&mut self, value: &[char]) -> &mut Self {
        self.option_announcer_chars = Vec::from(value);
        self
    }

    /// Specifies whether option codes are matched with case sensitivity.
    /// 
    /// Note that a [RegexOrText] object can override this setting for individual arguments
    ///
    /// Default: false
    pub fn option_codes_case_sensitive(&self) -> bool {
        self.option_codes_case_sensitive
    }

    /// Sets [option_codes_case_sensitive](Self::option_codes_case_sensitive)
    pub fn set_option_codes_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.option_codes_case_sensitive = value;
        self
    }

    /// Specifies whether option codes can be a string of length zero.
    /// 
    /// Having empty option codes means that a standalone option announcer char is an option argument in its own right.  This could
    /// be confusing however it is possible.
    /// 
    /// Default: false
    pub fn option_code_can_be_empty(&self) -> bool {
        self.option_code_can_be_empty
    }

    /// Sets [option_code_can_be_empty](Self::option_code_can_be_empty)
    pub fn set_option_code_can_be_empty(&mut self, value: bool) -> &mut Self {
        self.option_code_can_be_empty = value;
        self
    }

    /// Specifies whether option codes with more than one character must be announced with 2 successive option announcer characters.
    /// 
    /// This convention is common but not necessary.
    /// 
    /// Default: false
    pub fn multi_char_option_code_requires_double_announcer(&self) -> bool {
        self.multi_char_option_code_requires_double_announcer
    }

    /// Sets [multi_char_option_code_requires_double_announcer](Self::multi_char_option_code_requires_double_announcer)
    pub fn set_multi_char_option_code_requires_double_announcer(&mut self, value: bool) -> &mut Self {
        self.multi_char_option_code_requires_double_announcer = value;
        self
    }

    /// Get the array of characters any of which can be used end an option code and announce its option value.
    ///
    /// If an option argument does not end with this character, then it is a switch/flag only and does not include a value.
    /// If it does contain this character, then the characters prior to this character are the option code and the characters after
    /// it, are the option value.
    /// 
    /// Note that if a whitespace character is used as a option value announcer, there is some ambiguity as to whether that character is
    /// announcing the value for that option or being a delimiter for the next argument.  This ambiguity is resolved by a matcher's
    /// [`Matcher.option_has_value`](Matcher::option_has_value) property.
    ///
    /// Default: `[' ']`  (Array with one space character)
    pub fn option_value_announcer_chars(&self) -> &[char] {
        &self.option_value_announcer_chars
    }

    /// Set the array of characters any of which can be used end an option code and announce its option value. See
    /// [Parser::option_value_announcer_chars](Parser::option_value_announcer_chars) for more details.
    pub fn set_option_value_announcer_chars(&mut self, value: &[char]) -> &mut Self {
        self.option_value_announcer_chars = Vec::from(value);
        self
    }

    /// Specifies whether option values are matched with case sensitivity.
    /// 
    /// Note that a [RegexOrText] object can override this setting for individual arguments.
    /// 
    /// Default: false
    pub fn option_values_case_sensitive(&self) -> bool {
        self.option_values_case_sensitive
    }

    /// Sets [option_values_case_sensitive](Self::option_values_case_sensitive)
    pub fn set_option_values_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.option_values_case_sensitive = value;
        self
    }

    /// Specifies whether parameters are matched with case sensitivity.
    /// 
    /// Note that a [RegexOrText] object can override this setting for individual arguments.
    /// 
    /// Default: false
    pub fn params_case_sensitive(&self) -> bool {
        self.params_case_sensitive
    }

    /// Sets [params_case_sensitive](Self::params_case_sensitive)
    pub fn set_params_case_sensitive(&mut self, value: bool) -> &mut Self {
        self.params_case_sensitive = value;
        self
    }

    /// Specifies whether quote characters can be embedded in a quoted parameter or option value by using double quotes.
    /// 
    /// If true, two successive quote characters in quoted text will be treated as one embedded quote character within
    /// the text.
    /// 
    /// Line Default: true\
    /// Env args Default: false
    pub fn embed_quote_char_with_double(&self) -> bool {
        self.embed_quote_char_with_double
    }

    /// Set [embed_quote_char_with_double](Self::embed_quote_char_with_double)
    pub fn set_embed_quote_char_with_double(&mut self, value: bool) -> &mut Self {
        self.embed_quote_char_with_double = value;
        self
    }

    /// The escape character within a parameter or option value, signifies that the subsequent character should be
    /// treated as a literal character and not have a special purpose.
    /// 
    /// Some characters have special purposes in the command line. For example, a quote character normally specifies
    /// the start or end of the parameter/option value. To embed it within this quoted value, a user can place
    /// an escape character before it. The quote character will then be treated as a literal character and not
    /// as the character used to quote a string.
    /// 
    /// Only characters specified by [escapeable_logical_chars](Self::escapeable_logical_chars) or
    /// [escapeable_chars](Self::escapeable_chars) can be escaped. A parsing error will be returned if the command
    /// line attempts to escape any other character.
    /// 
    /// Note that if escaping is enabled, it is recommended that the `Escape` logical character itself also be escaped.
    /// That allows a user to include the escape character as a literal in a parameter or option value.
    ///
    /// Default: `None` (no escaping of characters)
    pub fn escape_char(&self) -> &Option<char> {
        &self.escape_char
    }

    /// Set [escape_char](Self::escape_char).
    pub fn set_escape_char(&mut self, value: Option<char>) -> &mut Self {
        self.escape_char = value;
        self
    }

    /// Set [escape_char](Self::escape_char).
    pub fn some_escape_char(&mut self, value: char) -> &mut Self {
        self.escape_char = Some(value);
        self
    }

    /// Set [escape_char](Self::escape_char) to `None` which disables character escaping.
    pub fn none_escape_char(&mut self) -> &mut Self {
        self.escape_char = None;
        self
    }

    /// The array of [logical characters](EscapeableLogicalChar) which the command line can escape.
    /// 
    /// Default: [[EscapeableLogicalChar::Escape](EscapeableLogicalChar::Escape),
    /// [EscapeableLogicalChar::Quote](EscapeableLogicalChar::Quote)]
    pub fn escapeable_logical_chars(&self) -> &[EscapeableLogicalChar] {
        &self.escapeable_logical_chars
    }

    /// Set [escapeable_logical_chars](Self::escapeable_logical_chars)
    pub fn set_escapeable_logical_chars(&mut self, value: &[EscapeableLogicalChar]) -> &mut Self {
        self.escapeable_logical_chars = Vec::from(value);
        self
    }

    /// The array of literal characters which the command line can escape.
    /// 
    /// Normally, a character which is not a special purpose character does not need to be escaped. Accordingly,
    /// this property would generally be left as an empty array.
    /// 
    /// Default: []
    pub fn escapeable_chars(&self) -> &[char] {
        &self.escapeable_chars
    }

    /// Set [escapeable_chars](Self::escapeable_chars)
    pub fn set_escapeable_chars(&mut self, value: &[char]) -> &mut Self {
        self.escapeable_chars = Vec::from(value);
        self
    }

    /// Whether the first argument should be interpretted as the binary's name.
    /// 
    /// Operating systems and shells traditionally insert the name or path of a binary as the first parameter of a
    /// command line (or environmental arguments) passed to an application. When this property is true,
    /// the parser will treat the first parameter accordingly.
    /// 
    /// However if an application internal command is parsed then the first parameter will most likely not
    /// be the binary name. This property can be set to false so the first parameter is treated like all other
    /// parameters.
    /// 
    /// Note that for security purposes, if the operating system inserts the first argument, it should not be relied
    /// upon to be the binary path.  It is possible for arbitrary text to be passed as the first parameter.
    pub fn first_arg_is_binary(&self) -> bool {
        self.first_arg_is_binary
    }

    /// Set [first_arg_is_binary](Self::first_arg_is_binary)
    pub fn set_first_arg_is_binary(&mut self, value: bool) -> &mut Self {
        self.first_arg_is_binary = value;
        self
    }

    /// Get the array of characters which terminate the parsing of arguments in the command line.
    /// 
    /// If any of the characters in this array are encountered outside a quoted value, then that character
    /// and all remaining characters in the command line are ignored.  This can be used to ignore standard input/output
    /// redirection and the end of a command line.
    ///
    /// Default: `[]`  (Empty array)
    pub fn parse_terminate_chars(&self) -> &[char] {
        &self.parse_terminate_chars
    }

    /// Set [parse_terminate_chars](Self::parse_terminate_chars)
    pub fn set_parse_terminate_chars(&mut self, value: &[char]) -> &mut Self {
        self.parse_terminate_chars = Vec::from(value);
        self
    }

}

impl<O: Default, P: Default> Parser<O, P> {

    /// Set Parser properties to their default values for parsing a command line.
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

    /// Set Parser properties to their default values for parsing environmental arguments.
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

    /// The array of [matchers](Matcher) registered with the Parser.
    pub fn matchers(&self) -> &Matchers<O, P> {
        &self.matchers
    }

    /// Create and return a new [matcher](Matcher) which has been added to the end of the Parser's list of matchers.
    pub fn push_new_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new(name);
        self.push_matcher(matcher)
    }

    /// Create and return a new [matcher](Matcher) for option arguments.  The matcher has been added to the end of the Parser's
    /// list of matchers.
    pub fn push_new_option_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new_option(name);
        self.push_matcher(matcher)
    }

    /// Create and return a new [matcher](Matcher) for parameter arguments.  The matcher has been added to the end of the Parser's
    /// list of matchers.
    pub fn push_new_param_matcher(&mut self, name: &str) -> &mut Matcher<O, P> {
        let matcher: Matcher<O, P> = Matcher::new_param(name);
        self.push_matcher(matcher)
    }

    /// Add a supplied [matcher](Matcher) to the end of the Parser's list of matchers. The Parser will take ownership of this matcher.
    pub fn push_matcher(&mut self, mut matcher: Matcher<O, P>) -> &mut Matcher<O, P> {
        let index = self.matchers.len();
        matcher.set_index(index);
        self.matchers.push(matcher);
        &mut self.matchers[index]
    }

    /// Delete the first [matcher](Matcher) in the list whose name equals the value of the `name` parameter.
    /// 
    /// Returns true if a matcher was deleted otherwise returns false.
    pub fn delete_matcher(&mut self, name: &str) -> bool {
        if let Some(matcher) = self.find_matcher(name) {
            let idx = matcher.index();
            self.delete_matcher_at(idx);
            true
        } else {
            false
        }
    }

    /// Delete the [matcher](Matcher) at the position in the list specified by the `index` parameter.
    pub fn delete_matcher_at(&mut self, index: usize) {
        self.matchers.remove(index);
    }

    /// Delete all [matcher](Matcher)s from the Parser's list of matchers.
    pub fn clear_matchers(&mut self) {
        self.matchers.clear();
    }

    /// Find and return the first [matcher](Matcher) in the list whose name equals the value of the `name` parameter.
    /// 
    /// Returns a reference to the matcher in an option if found.  Otherwise return `None`.
    pub fn find_matcher(&self, name: &str) -> Option<&Matcher<O, P>> {
        self.matchers.iter().find(|&matcher| matcher.name() == name)
    }

    /// Parse this applications environmental arguments.
    /// 
    /// If successful, returns a success result holding an array of the parsed arguments. Otherwise return an error result
    /// containing a [ParseError](ParseError) struct which holds the error details.
    pub fn parse_env(&self) -> Result<Args<O, P>, ParseError> {
        self.parse_env_args(env::args())
    }

    /// Parse the environmental arguments passed in the `env_args` parameter.
    /// 
    /// If successful, returns a success result holding an array of the [parsed arguments](Args). Otherwise return an error result
    /// containing a [ParseError](ParseError) struct which holds the error details.
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
                    parse_state.increment_env_line_approximate_char_idx();
                }
            }

            if more {
                parse_state.env_arg_idx = env_arg_idx;
                parse_state.line_or_env_arg = env_arg.clone();
                parse_state.line_or_env_arg_char_idx = 0;
    
                for unicode_char in env_arg.chars() {
                    let env_char = EnvChar::Unicode(unicode_char);
                    more = self.process_char(&mut parse_state, env_char, &mut args)?;

                    if more {
                        parse_state.increment_env_arg_char_idx();
                        parse_state.increment_env_line_approximate_char_idx();
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

    /// Parse a command line.
    /// 
    /// If successful, returns a success result holding an array of the [parsed arguments](Args). Otherwise return an error result
    /// containing a [ParseError](ParseError) struct which holds the error details.
    pub fn parse_line(&self, line: &str) -> Result<Args<O, P>, ParseError> {
        let mut args = Vec::new();

        let mut parse_state = ParseState::new(
            line,
            self.first_arg_is_binary,
            self.multi_char_option_code_requires_double_announcer,
        );

        parse_state.line_or_env_arg_char_idx = 0;

        for char in line.chars() {
            let env_char = EnvChar::Unicode(char);
            let more = self.process_char(&mut parse_state, env_char, &mut args)?;

            if more {
                parse_state.increment_env_line_approximate_char_idx();
                parse_state.increment_env_arg_char_idx();
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
                            self.initialise_option_parsing(parse_state, unicode_char);
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
                        if parse_state.value_quoted {
                            Err(parse_state.create_param_error(ParseErrorTypeId::ParamMissingClosingQuoteCharacter))
                        } else {
                            self.match_param_arg(parse_state, args)?;
                            parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                            Ok(true)
                        }
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

                        Ok(true)
                    }
                }
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
                                Err(parse_state.create_param_error(ParseErrorTypeId::QuotedParamNotFollowedByWhitespaceChar))?;
                            }
                        }
                    }
                }
                Ok(true)
            }

            ArgParseState::InParamEscaped => {
                match env_char {
                    EnvChar::Separator => {
                        Err(parse_state.create_param_error(ParseErrorTypeId::EscapeCharacterAtEndOfParam))?;
                    }
                    EnvChar::Unicode(unicode_char) => {
                        if self.can_char_be_escaped(parse_state, unicode_char) {
                            parse_state.value_bldr.push(unicode_char);
                            parse_state.arg_parse_state = ArgParseState::InParam;
                        } else {
                            Err(parse_state.create_param_error(ParseErrorTypeId::EscapedCharacterInParamCannotBeEscaped))?;
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
                                            Err(parse_state.create_option_error(ParseErrorTypeId::OptionCodeCannotContainQuoteChar))
                                        } else {
                                            if let Some(escape_char) = self.escape_char {
                                                if escape_char == unicode_char {
                                                    Err(parse_state.create_option_error(ParseErrorTypeId::OptionCodeCannotContainEscapeChar))
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
                                if parse_state.value_quoted {
                                    Err(parse_state.create_option_error(ParseErrorTypeId::OptionValueMissingClosingQuoteCharacter))
                                } else {
                                    self.match_option_arg(parse_state, true, args)?;
                                    parse_state.arg_parse_state = ArgParseState::WaitOptionOrParam;
                                    Ok(true)
                                }
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

                                Ok(true)
                            },
                        }
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
                                        Err(parse_state.create_option_error(ParseErrorTypeId::QuotedOptionValueNotFollowedByWhitespaceChar))?;
                                    }
                                }
                            },
                        }
                        Ok(true)
                    }
                    OptionParseState::InValueEscaped => {
                        match env_char {
                            EnvChar::Separator => {
                                Err(parse_state.create_option_error(ParseErrorTypeId::EscapeCharacterAtEndOfOptionValue))?;
                            }
                            EnvChar::Unicode(unicode_char) => {
                                if self.can_char_be_escaped(parse_state, unicode_char) {
                                    parse_state.value_bldr.push(unicode_char);
                                    parse_state.option_parse_state = OptionParseState::InValue;
                                } else {
                                    Err(parse_state.create_option_error(ParseErrorTypeId::EscapedCharacterInOptionValueCannotBeEscaped))?;
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
        parse_state.set_option_code(Some(parse_state.line_or_env_arg_char_idx))?;
        match value_announced {
            ValueAnnounced::Definitely => {
                if self.can_option_code_have_value(parse_state) {
                    parse_state.option_parse_state = OptionParseState::WaitOptionValue;
                    Ok(())
                } else {
                    Err(parse_state.create_option_error(ParseErrorTypeId::NoMatchForOptionWithValue))
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
                    Err(parse_state.create_param_error(ParseErrorTypeId::ParamMissingClosingQuoteCharacter))
                } else {
                    self.match_param_arg(parse_state, args)
                }
            }

            ArgParseState::InParamPossibleEndQuote => {
                self.match_param_arg(parse_state, args)
            }

            ArgParseState::InParamEscaped => {
                Err(parse_state.create_param_error(ParseErrorTypeId::EscapedCharacterInParamCannotBeEscaped))
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
                                Err(parse_state.create_option_error(ParseErrorTypeId::OptionMissingValue))
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
                            Err(parse_state.create_option_error(ParseErrorTypeId::OptionValueMissingClosingQuoteCharacter))
                        } else {
                            self.match_option_arg(parse_state, true, args)
                        }
                    }
                    OptionParseState::InValuePossibleEndQuote => {
                        self.match_option_arg(parse_state, true, args)
                    }
                    OptionParseState::InValueEscaped => {
                        Err(parse_state.create_option_error(ParseErrorTypeId::EscapeCharacterAtEndOfLine))
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

    fn initialise_option_parsing(&self, parse_state: &mut ParseState, unicode_char: char) {
        parse_state.option_parse_state = OptionParseState::InCode;
        parse_state.option_announcer_char = unicode_char;
        parse_state.arg_start_char_idx = parse_state.line_or_env_arg_char_idx;
        parse_state.arg_start_env_line_approximate_char_idx = parse_state.env_line_approximate_char_idx;
        parse_state.option_code_start_line_char_idx = parse_state.line_or_env_arg_char_idx + 1;
    }

    fn initialise_param_parsing(&self, parse_state: &mut ParseState, unicode_char: char, is_binary: bool) {
        parse_state.value_bldr.clear();
        parse_state.arg_start_char_idx = parse_state.line_or_env_arg_char_idx;
        parse_state.arg_start_env_line_approximate_char_idx = parse_state.env_line_approximate_char_idx;
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
            *matcher.option_has_value() != OptionHasValue::Never
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
            match *matcher.option_has_value() {
                OptionHasValue::Always => {
                    if matcher.option_value_can_start_with_option_announcer() {
                        Ok(OptionHasValueBasedOnFirstChar::Must)
                    } else {
                        if first_char_of_value_is_option_announcer {
                            Err(parse_state.create_option_error(ParseErrorTypeId::OptionValueCannotStartWithOptionAnnouncer))
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
                            Err(parse_state.create_option_error(ParseErrorTypeId::OptionValueCannotStartWithOptionAnnouncer))
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
                    Err(parse_state.create_option_error(ParseErrorTypeId::UnmatchedOption))
                }
            } else {
                Err(parse_state.create_option_error(ParseErrorTypeId::UnmatchedOption))
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
            char_index: parse_state.arg_start_char_idx,
            env_line_approximate_char_index: parse_state.arg_start_env_line_approximate_char_idx,
            arg_index: parse_state.arg_count,
            env_arg_index: parse_state.env_arg_idx,
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
            match *matcher.option_has_value() {
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
                Err(parse_state.create_param_error(ParseErrorTypeId::UnmatchedParam))
            }
        }
    }

    fn add_param_arg<'a>(&self, parse_state: &mut ParseState, matcher: &'a Matcher<O, P>, args: &mut Args<'a, O, P>) {
        let properties = ParamProperties {
            matcher,
            char_index: parse_state.arg_start_char_idx,
            env_line_approximate_char_index: parse_state.arg_start_env_line_approximate_char_idx,
            arg_index: parse_state.arg_count,
            env_arg_index: parse_state.env_arg_idx,
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
            char_index: parse_state.arg_start_char_idx,
            env_line_approximate_char_index: parse_state.arg_start_env_line_approximate_char_idx,
            arg_index: parse_state.arg_count,
            env_arg_index: parse_state.env_arg_idx,
            value_text: parse_state.value_bldr.clone(),
        };

        let arg = Arg::Binary(properties);
        args.push(arg);

        parse_state.arg_count += 1;
    }

    fn try_match_option_excluding_value(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, matcher.arg_indices())
        &&
        self.try_match_arg_type(MatchArgTypeId::Option, matcher.arg_type())
        &&
        self.try_match_index(&parse_state.option_count, matcher.option_indices())
        &&
        self.try_match_option_code(&parse_state.option_code, matcher.option_codes())
    }

    fn try_match_param(&self, parse_state: &ParseState, matcher: &Matcher<O, P>) -> bool {
        self.try_match_index(&parse_state.arg_count, matcher.arg_indices())
        &&
        self.try_match_arg_type(MatchArgTypeId::Param, matcher.arg_type())
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

    fn try_match_arg_type(&self, value: MatchArgTypeId, matcher_value: &Option<MatchArgTypeId>) -> bool {
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

/// A logical character is either a group of characters (eg whitespace characters) or a special purpose
/// character which is configured by the parser (eg Quote character).
#[derive(Clone)]
pub enum EscapeableLogicalChar {
    /// Escape character.
    Escape,
    /// Quote character.
    Quote,
    /// Any whitespace character.
    Whitespace,
    /// Option announcer character.
    OptionAnnouncer,
    /// Option value announcer character.
    OptionValueAnnouncer,
    /// Any character.
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
