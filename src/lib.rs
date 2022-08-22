//! A command line parser for Rust
//! 
//! The primary purpose of this library is to parse a full command line. That is, a string containing the complete command line.
//! It can also parse rust environment command line arguments as supplied by Rust (`std::env::Args`) however there are other libraries which
//! specialise in this and may be a better choice.
//! 
//! While the Rust standard library does (currently) not give access to the full environment command line, this library could be used to
//! parse internal commands entered within an application.
//! 
//! Follow the steps below to parse a command line:
//! 1. Create an enum with one variant for each type of option argument expected in the command line.
//! 1. Create an enum with one variant for each type of parameter argument expected in the command line.
//! 1. Create an instance of `parmacl::Parser`.
//! 1. Set the properties of the Parser instance to reflect the style of the command line.
//! 1. Add a matcher for all possible arguments to the parser. Tag each matcher with the appropriate enum.
//! 1. Call `Parser.parse_line(command_line)` which will parse the command line and return a result containing either an array of arguments or
//! an error.
//! 1. Loop through returned arguments and process each. The arguments are ordered by appearance in the command line.
//! 
//! Example
//! 
//! ```
//!use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
//!
//!const LINE: &str = r#""binary name" -a "1st ""Param""" -B optBValue "2nd Param" -c "C OptValue""#;
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
//!        .some_option_codes(&[RegexOrText::new_text("a")]);
//! 
//!parser
//!    .push_new_option_matcher("optionB")
//!        .set_option_tag(OptionEnum::B)
//!        .some_option_codes(&[RegexOrText::new_text("b")])
//!        .some_option_has_value(OptionHasValue::IfPossible);
//! 
//!parser
//!    .push_new_option_matcher("optionC")
//!        .set_option_tag(OptionEnum::C)
//!        .some_option_codes(&[RegexOrText::new_text("c")])
//!        .some_option_has_value(OptionHasValue::Always);
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
//!            assert_eq!(properties.line_char_index, 0);
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
//!                    assert_eq!(properties.line_char_index, 14);
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
//!                    assert_eq!(properties.line_char_index, 17);
//!                },
//!                ParamEnum::Param2 => {
//!                    // Process parameter Param2
//!                },
//!            }
//!        }
//!    }
//!}
//!```

#![allow(clippy::collapsible_else_if)]

mod env_char;
mod parse_error_id;
mod parse_error;
mod regex_or_text;
mod matcher;
mod arg;
mod parser;

mod parse_state;

pub use parse_error_id::{
    ParseErrorId,
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
    MatchArgType,
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
