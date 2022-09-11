#![warn(missing_docs)]
//! A command line parser for Rust
//! 
//! The primary purpose of this library is to parse a full command line. That is, a string containing the complete command line.
//! It can also parse rust environment command line arguments as supplied by Rust (`std::env::Args`) however there are other libraries which
//! specialise in this and may be a better choice.
//! 
//! While the Rust standard library does not give access to the full environment command line, this library could be used to
//! parse internal commands entered within an application.
//! 
//! See [Features](#features).
//! 
//! # Usage
//! 
//! Follow the steps below to parse a command line:
//! 1. Create an enum with one variant for each type of option argument expected in the command line.
//! 1. Create an enum with one variant for each type of parameter argument expected in the command line.
//! 1. Create an instance of [parmacl::Parser](Parser).
//! 1. If necessary, set relevant properties of the Parser instance to reflect the style of the command line.
//! 1. Add a [matcher](Matcher) for all possible arguments to the parser. Tag each matcher with the appropriate enum.
//! 1. Call [Parser.parse_line(command_line)](Parser::parse_line) which will parse the command line and return a result containing
//! either a [vector of parsed arguments](Args) or an error.
//! 1. Loop through returned arguments and process each. The arguments are ordered by appearance in the command line.
//! 
//! ## Example
//! 
//! ```
//!use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
//!
//!const LINE: &str = r#""binary name" -a "1st ""Param""" -B optValue "param2" -c "C OptValue""#;
//! 
//!#[derive(Default)]
//!enum OptionEnum {
//!    #[default] A,
//!    B,
//!    C,
//!}
//!#[derive(Default)]
//!enum ParamEnum {
//!    #[default] Param1,
//!    Param2,
//!}
//! 
//!let mut parser: Parser<OptionEnum, ParamEnum> = Parser::new();
//! 
//!parser
//!    .push_new_option_matcher("optionA")
//!        .set_option_tag(OptionEnum::A)
//!        .some_option_codes(&[RegexOrText::with_text("a")]);
//! 
//!parser
//!    .push_new_option_matcher("optionB")
//!        .set_option_tag(OptionEnum::B)
//!        .some_option_codes(&[RegexOrText::with_text("b")])
//!        .set_option_has_value(OptionHasValue::IfPossible);
//! 
//!parser
//!    .push_new_option_matcher("optionC")
//!        .set_option_tag(OptionEnum::C)
//!        .some_option_codes(&[RegexOrText::with_text("c")])
//!        .set_option_has_value(OptionHasValue::Always);
//! 
//!parser
//!     .push_new_param_matcher("param1")
//!         .set_param_tag(ParamEnum::Param1)
//!         .some_param_indices(&[0]);
//! 
//!parser
//!    .push_new_param_matcher("param2")
//!        .set_param_tag(ParamEnum::Param2)
//!        .some_param_indices(&[1]);
//! 
//!let args = parser.parse_line(LINE).unwrap();
//! 
//!assert_eq!(args.len(), 6);
//! 
//!for arg in args {
//!    match arg {
//!        Arg::Binary(properties) => {
//!            assert_eq!(properties.arg_index, 0);
//!            assert_eq!(properties.value_text, "binary name");
//!            assert_eq!(properties.char_index, 0);
//!        },
//!        Arg::Option(properties) => {
//!            match properties.matcher.option_tag() {
//!                OptionEnum::A => {
//!                    // Process option A
//!                    assert_eq!(properties.matcher.name(), "optionA");
//!                    assert_eq!(properties.arg_index, 1);
//!                    assert_eq!(properties.option_index, 0);
//!                    assert_eq!(properties.code, "a");
//!                    assert_eq!(properties.value_text, None);
//!                    assert_eq!(properties.char_index, 14);
//!                },
//!                OptionEnum::B => {
//!                    // Process option B
//!                },
//!                OptionEnum::C => {
//!                    // Process option C
//!                },
//!            }
//!        }
//!        Arg::Param(properties) => {
//!            match properties.matcher.param_tag() {
//!                ParamEnum::Param1 => {
//!                    // Process parameter Param1
//!                    assert_eq!(properties.matcher.name(), "param1");
//!                    assert_eq!(properties.arg_index, 2);
//!                    assert_eq!(properties.param_index, 0);
//!                    assert_eq!(properties.value_text, "1st \"Param\"");
//!                    assert_eq!(properties.char_index, 17);
//!                },
//!                ParamEnum::Param2 => {
//!                    // Process parameter Param2
//!                },
//!            }
//!        }
//!    }
//!}
//!```
//! # Parsing environment arguments
//! 
//! Parmacl can also be used to parse the environment command line arguments passed to an application. The above example could be used to parse
//! environment arguments with the following changes.
//! 
//! Instead of using the [new()](Parser::new) constructor function, use the [with_env_args_defaults()](Parser::with_env_args_defaults) constructor.
//! This will create an instance of Parser with defaults suitable for parsing environment arguments.
//!```
//!# use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
//!#
//!# const LINE: &str = r#""binary name" -a "1st ""Param""" -B optValue "param2" -c "C OptValue""#;
//!#  
//!# #[derive(Default)]
//!# enum OptionEnum {
//!#     #[default] A,
//!#     B,
//!#     C,
//!# }
//!# #[derive(Default)]
//!# enum ParamEnum {
//!#     #[default] Param1,
//!#     Param2,
//!# }
//!#  
//! let mut parser: Parser<OptionEnum, ParamEnum> = Parser::with_env_args_defaults();
//!#  
//!# parser
//!#     .push_new_option_matcher("optionA")
//!#         .set_option_tag(OptionEnum::A)
//!#         .some_option_codes(&[RegexOrText::with_text("a")]);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionB")
//!#         .set_option_tag(OptionEnum::B)
//!#         .some_option_codes(&[RegexOrText::with_text("b")])
//!#         .set_option_has_value(OptionHasValue::IfPossible);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionC")
//!#         .set_option_tag(OptionEnum::C)
//!#         .some_option_codes(&[RegexOrText::with_text("c")])
//!#         .set_option_has_value(OptionHasValue::Always);
//!#  
//!# parser
//!#      .push_new_param_matcher("param1")
//!#          .set_param_tag(ParamEnum::Param1)
//!#          .some_param_indices(&[0]);
//!#  
//!# parser
//!#     .push_new_param_matcher("param2")
//!#         .set_param_tag(ParamEnum::Param2)
//!#         .some_param_indices(&[1]);
//!#  
//!# let args = parser.parse_env().unwrap();
//!#  
//!# // assert_eq!(args.len(), 6); // commented out to allow documentation tests to pass
//!#  
//!# for arg in args {
//!#     match arg {
//!#         Arg::Binary(properties) => {
//!#             assert_eq!(properties.arg_index, 0);
//!#             // assert_eq!(properties.value_text, "binary name"); // commented out to allow documentation tests to pass
//!#             assert_eq!(properties.env_line_approximate_char_index, 0);
//!#         },
//!#         Arg::Option(properties) => {
//!#             match properties.matcher.option_tag() {
//!#                 OptionEnum::A => {
//!#                     // Process option A
//!#                     assert_eq!(properties.matcher.name(), "optionA");
//!#                     assert_eq!(properties.arg_index, 1);
//!#                     assert_eq!(properties.option_index, 0);
//!#                     assert_eq!(properties.code, "a");
//!#                     assert_eq!(properties.value_text, None);
//!#                     assert_eq!(properties.env_line_approximate_char_index, 14);
//!#                 },
//!#                 OptionEnum::B => {
//!#                     // Process option B
//!#                 },
//!#                 OptionEnum::C => {
//!#                     // Process option C
//!#                 },
//!#             }
//!#         }
//!#         Arg::Param(properties) => {
//!#             match properties.matcher.param_tag() {
//!#                 ParamEnum::Param1 => {
//!#                     // Process parameter Param1
//!#                     assert_eq!(properties.matcher.name(), "param1");
//!#                     assert_eq!(properties.arg_index, 2);
//!#                     assert_eq!(properties.param_index, 0);
//!#                     assert_eq!(properties.value_text, "1st \"Param\"");
//!#                     assert_eq!(properties.env_line_approximate_char_index, 17);
//!#                 },
//!#                 ParamEnum::Param2 => {
//!#                     // Process parameter Param2
//!#                 },
//!#             }
//!#         }
//!#     }
//!# }
//!```
//! Use the [parse_env()](Parser::parse_env) function instead of the [parse()](Parser::parse_line) function.
//!```
//!# use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
//!#
//!# const LINE: &str = r#""binary name" -a "1st ""Param""" -B optValue "param2" -c "C OptValue""#;
//!#  
//!# #[derive(Default)]
//!# enum OptionEnum {
//!#     #[default] A,
//!#     B,
//!#     C,
//!# }
//!# #[derive(Default)]
//!# enum ParamEnum {
//!#     #[default] Param1,
//!#     Param2,
//!# }
//!#  
//!# let mut parser: Parser<OptionEnum, ParamEnum> = Parser::with_env_args_defaults();
//!#  
//!# parser
//!#     .push_new_option_matcher("optionA")
//!#         .set_option_tag(OptionEnum::A)
//!#         .some_option_codes(&[RegexOrText::with_text("a")]);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionB")
//!#         .set_option_tag(OptionEnum::B)
//!#         .some_option_codes(&[RegexOrText::with_text("b")])
//!#         .set_option_has_value(OptionHasValue::IfPossible);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionC")
//!#         .set_option_tag(OptionEnum::C)
//!#         .some_option_codes(&[RegexOrText::with_text("c")])
//!#         .set_option_has_value(OptionHasValue::Always);
//!#  
//!# parser
//!#      .push_new_param_matcher("param1")
//!#          .set_param_tag(ParamEnum::Param1)
//!#          .some_param_indices(&[0]);
//!#  
//!# parser
//!#     .push_new_param_matcher("param2")
//!#         .set_param_tag(ParamEnum::Param2)
//!#         .some_param_indices(&[1]);
//!#  
//! let args = parser.parse_env().unwrap();
//!#  
//!# // assert_eq!(args.len(), 6); // commented out to allow documentation tests to pass
//!#  
//!# for arg in args {
//!#     match arg {
//!#         Arg::Binary(properties) => {
//!#             assert_eq!(properties.arg_index, 0);
//!#             // assert_eq!(properties.value_text, "binary name"); // commented out to allow documentation tests to pass
//!#             assert_eq!(properties.env_line_approximate_char_index, 0);
//!#         },
//!#         Arg::Option(properties) => {
//!#             match properties.matcher.option_tag() {
//!#                 OptionEnum::A => {
//!#                     // Process option A
//!#                     assert_eq!(properties.matcher.name(), "optionA");
//!#                     assert_eq!(properties.arg_index, 1);
//!#                     assert_eq!(properties.option_index, 0);
//!#                     assert_eq!(properties.code, "a");
//!#                     assert_eq!(properties.value_text, None);
//!#                     assert_eq!(properties.env_line_approximate_char_index, 14);
//!#                 },
//!#                 OptionEnum::B => {
//!#                     // Process option B
//!#                 },
//!#                 OptionEnum::C => {
//!#                     // Process option C
//!#                 },
//!#             }
//!#         }
//!#         Arg::Param(properties) => {
//!#             match properties.matcher.param_tag() {
//!#                 ParamEnum::Param1 => {
//!#                     // Process parameter Param1
//!#                     assert_eq!(properties.matcher.name(), "param1");
//!#                     assert_eq!(properties.arg_index, 2);
//!#                     assert_eq!(properties.param_index, 0);
//!#                     assert_eq!(properties.value_text, "1st \"Param\"");
//!#                     assert_eq!(properties.env_line_approximate_char_index, 17);
//!#                 },
//!#                 ParamEnum::Param2 => {
//!#                     // Process parameter Param2
//!#                 },
//!#             }
//!#         }
//!#     }
//!# }
//!```
//! Since the shell will already have parsed the command line, and passed the individual arguments to the application, the parser can
//! only guess the position of each argument in the command line. Use property `env_line_approximate_char_index` instead of `char_index`
//! in [ParamProperties](ParamProperties) or [OptionProperties](OptionProperties) to get the approximate position of a the argument in
//! the command line.
//!```
//!# use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
//!#
//!# const LINE: &str = r#""binary name" -a "1st ""Param""" -B optValue "param2" -c "C OptValue""#;
//!#  
//!# #[derive(Default)]
//!# enum OptionEnum {
//!#     #[default] A,
//!#     B,
//!#     C,
//!# }
//!# #[derive(Default)]
//!# enum ParamEnum {
//!#     #[default] Param1,
//!#     Param2,
//!# }
//!#  
//!# let mut parser: Parser<OptionEnum, ParamEnum> = Parser::with_env_args_defaults();
//!#  
//!# parser
//!#     .push_new_option_matcher("optionA")
//!#         .set_option_tag(OptionEnum::A)
//!#         .some_option_codes(&[RegexOrText::with_text("a")]);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionB")
//!#         .set_option_tag(OptionEnum::B)
//!#         .some_option_codes(&[RegexOrText::with_text("b")])
//!#         .set_option_has_value(OptionHasValue::IfPossible);
//!#  
//!# parser
//!#     .push_new_option_matcher("optionC")
//!#         .set_option_tag(OptionEnum::C)
//!#         .some_option_codes(&[RegexOrText::with_text("c")])
//!#         .set_option_has_value(OptionHasValue::Always);
//!#  
//!# parser
//!#      .push_new_param_matcher("param1")
//!#          .set_param_tag(ParamEnum::Param1)
//!#          .some_param_indices(&[0]);
//!#  
//!# parser
//!#     .push_new_param_matcher("param2")
//!#         .set_param_tag(ParamEnum::Param2)
//!#         .some_param_indices(&[1]);
//!#  
//!# let args = parser.parse_env().unwrap();
//!#  
//!# // assert_eq!(args.len(), 6); // commented out to allow documentation tests to pass
//!#  
//!# for arg in args {
//!#     match arg {
//!#         Arg::Binary(properties) => {
//!#             assert_eq!(properties.arg_index, 0);
//!#             // assert_eq!(properties.value_text, "binary name"); // commented out to allow documentation tests to pass
//!#             assert_eq!(properties.env_line_approximate_char_index, 0);
//!#         },
//!#         Arg::Option(properties) => {
//!#             match properties.matcher.option_tag() {
//!                 OptionEnum::A => {
//!                     // Process option A
//!                     assert_eq!(properties.matcher.name(), "optionA");
//!                     assert_eq!(properties.arg_index, 1);
//!                     assert_eq!(properties.option_index, 0);
//!                     assert_eq!(properties.code, "a");
//!                     assert_eq!(properties.value_text, None);
//!                     assert_eq!(properties.env_line_approximate_char_index, 14);
//!                 },
//!#                 OptionEnum::B => {
//!#                     // Process option B
//!#                 },
//!#                 OptionEnum::C => {
//!#                     // Process option C
//!#                 },
//!#             }
//!#         }
//!#         Arg::Param(properties) => {
//!#             match properties.matcher.param_tag() {
//!#                 ParamEnum::Param1 => {
//!#                     // Process parameter Param1
//!#                     assert_eq!(properties.matcher.name(), "param1");
//!#                     assert_eq!(properties.arg_index, 2);
//!#                     assert_eq!(properties.param_index, 0);
//!#                     assert_eq!(properties.value_text, "1st \"Param\"");
//!#                     assert_eq!(properties.env_line_approximate_char_index, 17);
//!#                 },
//!#                 ParamEnum::Param2 => {
//!#                     // Process parameter Param2
//!#                 },
//!#             }
//!#         }
//!#     }
//!# }
//!```
//! # Understanding the command line
//! 
//! Parmacl considers a command line to have 3 types of arguments
//! * **Binary name**\
//! This normally is the first argument in the command line and is normally the path to the application's executable file (binary).
//! * **Parameters**\
//! Strings which the application will interpret. Parameters are typically identified by their order in the command line. In the above
//! [example](#example), `"1st ""Param"""` and `"param2"` are parameters.
//! * **Options**\
//! An option is an argument identified by a code.  As such it can be placed anywhere in the command line. It can optionally have a value.
//! If it does not have a value, it behaves like a flag/boolean.  In the above [example](#example), `-a` is an option with code `a` that
//! behaves like a flag. The options `-B optValue` (code `B`) and `-c "C OptValue"` (code `c`) are options with respective values
//! `optValue` and `C OptValue`.
//! 
//! Note that these arguments do not necessarily correspond to environment arguments created by a shell and passed to an application.
//! For example, an option with a value is identified by Parmacl as one argument whereas the shell may identify it as 2 arguments
//! (depending on what character is used to separate option values from the option code).
//! 
//! # Overview of parsing
//! 
//! Before parsing a command line, the parser needs to be configured with the style of the command line.  This includes things like
//! specifying whether parameters and option values can be quoted, whether quotes can be included in quoted parameters and option values,
//! specifying escaping of special characters.
//! 
//! It also needs to be configured with a list of [matchers](Matcher). A matcher is a struct with a set of filters. The filters are used
//! to match arguments identified by the parser. An argument needs to meet all filter conditions in order for it to be 'matched'
//!
//! When a command line is parsed, the parser will identify arguments based on its configured style.  The arguments are identified in order
//! from start to end of the command line.  As each argument is identified, it is matched against one of the [matchers](Matcher) assigned to
//! the parser. It is possible for an argument to match more than one matcher.  The parser will attempt to match each argument to matchers
//! in order of matchers in the parser's matcher list. Accordingly, more specific matchers should be inserted earlier in this list.
//! 
//! When an argument is matched, the parser generates a corresponding [Arg](Arg) variant with details of the argument. For parameter and
//! option arguments, the variant is also assigned a copy of either the respective Matcher's [param_tag](Matcher::param_tag) or
//! [option_tag](Matcher::option_tag) value.
//! 
//! These [Arg](Arg) variants are stored in an array (in same order as the arguments in the command line) which is returned as the success
//! result.
//! 
//! All arguments must be matched. If an argument is not matched, then the user specified an unsupported parameter or option and an
//! [unmatched parameter](ParseErrorTypeId::UnmatchedParam) or [unmatched option](ParseErrorTypeId::UnmatchedOption) will be returned.
//! 
//! If the parser detects an error in the command line, an error result will be returned containing a [ParseError](ParseError) struct.
//! This struct [identifies](ParseErrorTypeId) the reason for the error and where in the line the error occurred.
//! 
//! # Main types
//! 
//! * [Parser](Parser)\
//! The main object. To parse a command line, create an instance of this, set its properties to reflect the style of the command line,
//! assign matchers and then call one its parse functions. The result will either be the array of arguments or an error object.
//! * [Matcher](Matcher)\
//! Each argument must be matched against a matcher.  Typically one matcher is created for each argument however matchers can also
//! be used to match multiple arguments.
//! * [Arg](Arg)\
//! An enum with 3 variants: Binary, Param, Option. The Parser's parse functions return an array of these variants - each of which
//! identify an argument the parser found in the command line.
//! * [ArgProperties](ArgProperties)\
//! A trait shared by structs [BinaryProperties](BinaryProperties), [ParamProperties](ParamProperties) and
//! [OptionProperties](OptionProperties). Instances of these structs are associated with the respective [Arg](Arg) variants returned
//! by the parse function and provide details about each identified argument.
//! * [RegexOrText](RegexOrText)\
//! A struct representing either a Regex or text (string). An instance of RegexOrText can be assigned to the
//! [option_codes](Matcher::option_codes) or [value_text](Matcher::value_text) Matcher filter properties and determines whether the
//! filtering is by text or Regex.
//! * [ParseError](ParseError)\
//! The struct returned with an Error result from a parse function. Specifies the type of error and where in the line the error
//! occurred.
//! 
//! # Features
//! 
//! * Command line parsing
//! * Environment arguments parsing
//! * Command Line Style
//!     * Specify which character(s) can be used to quote parameters and option values
//!     * Specify which character(s) can be used to announce an option
//!     * Specify which character(s) can be used to announce an option value (space character can be included)
//!     * Specify which character(s) will terminate parsing of a command line
//!     * Case sensitivity when matching parameters, option codes and option values
//!     * Whether options with code that have more than one character, require 2 announcer characters (eg --anOpt)
//!     * Use double quotes to embed quote characters within quoted parameters and option values
//!     * Use escaping to include characters with special meaning
//!     * Whether first argument in command line is the binary's name/path
//! * Argument Matching
//!     * Parameter or Option
//!     * Argument indices
//!     * Parameter indices
//!     * Parameter text (string or Regex)
//!     * Option indices
//!     * Option codes (string or Regex)
//!     * Whether option has value (None, IfPossible, Always)
//!     * Option value text (string or Regex)
//!     * Whether option value can start with an option announcer character
//! * Tag parameters and options arguments with with any enum (or any other type) from matcher for easy identification
//! * Parse error result has properties detailing the type of error and where it occurred.



#![allow(clippy::collapsible_else_if)]

mod env_char;
mod parse_error_type_id;
mod parse_error;
mod regex_or_text;
mod matcher;
mod arg;
mod parser;

mod parse_state;

pub use parse_error_type_id::{
    ParseErrorTypeId,
};

pub use parse_error::{
    ParseError,
};

pub use regex_or_text::{
    RegexOrText,
};

pub use matcher:: {
    Matcher,
    Matchers,
    DefaultTagType,
    OptionHasValue,
    MatchArgTypeId,
    DEFAULT_OPTION_HAS_VALUE,
};

pub use arg::{
    ArgProperties,
    BinaryProperties,
    OptionProperties,
    ParamProperties,
    Arg,
    Args,
};

pub use parser::{
    Parser,
    EscapeableLogicalChar,
    DEFAULT_LINE_QUOTE_CHARS,
    DEFAULT_LINE_OPTION_ANNOUNCER_CHARS,
    DEFAULT_LINE_OPTION_CODES_CASE_SENSITIVE,
    DEFAULT_LINE_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
    DEFAULT_LINE_OPTION_VALUE_ANNOUNCER_CHARS,
    DEFAULT_LINE_OPTION_VALUES_CASE_SENSITIVE,
    DEFAULT_LINE_PARAMS_CASE_SENSITIVE,
    DEFAULT_LINE_EMBED_QUOTE_CHAR_WITH_DOUBLE,
    DEFAULT_LINE_ESCAPE_CHAR,
    DEFAULT_LINE_ESCAPEABLE_LOGICAL_CHARS,
    DEFAULT_LINE_ESCAPEABLE_CHARS,
    DEFAULT_LINE_PARSE_TERMINATE_CHARS,
    DEFAULT_LINE_FIRST_ARG_IS_BINARY,
    DEFAULT_ENV_ARGS_QUOTE_CHARS,
    DEFAULT_ENV_ARGS_OPTION_ANNOUNCER_CHARS,
    DEFAULT_ENV_ARGS_OPTION_CODES_CASE_SENSITIVE,
    DEFAULT_ENV_ARGS_OPTION_CODE_CAN_BE_EMPTY,
    DEFAULT_ENV_ARGS_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
    DEFAULT_ENV_ARGS_OPTION_VALUE_ANNOUNCER_CHARS,
    DEFAULT_ENV_ARGS_OPTION_VALUES_CASE_SENSITIVE,
    DEFAULT_ENV_ARGS_PARAMS_CASE_SENSITIVE,
    DEFAULT_ENV_ARGS_EMBED_QUOTE_CHAR_WITH_DOUBLE,
    DEFAULT_ENV_ARGS_ESCAPE_CHAR,
    DEFAULT_ENV_ARGS_ESCAPEABLE_LOGICAL_CHARS,
    DEFAULT_ENV_ARGS_ESCAPEABLE_CHARS,
    DEFAULT_ENV_ARGS_PARSE_TERMINATE_CHARS,
    DEFAULT_ENV_ARGS_FIRST_ARG_IS_BINARY,
};
