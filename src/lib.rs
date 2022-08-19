//! Simple Command Line Argument Parser for Rust
//! 
//! A library which uses matchers to parse a command line
//! 
//! This extra

#![allow(clippy::collapsible_else_if)]

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
    DEFAULT_QUOTE_CHAR,
    DEFAULT_OPTION_ANNOUNCER_CHARS,
    DEFAULT_OPTION_CODES_CASE_SENSITIVE,
    DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
    DEFAULT_OPTION_VALUE_ANNOUNCER_CHARS,
    DEFAULT_OPTION_VALUES_CASE_SENSITIVE,
    DEFAULT_PARAMS_CASE_SENSITIVE,
    DEFAULT_EMBED_QUOTE_CHAR_WITH_DOUBLE,
    DEFAULT_ESCAPE_CHAR,
    DEFAULT_ESCAPEABLE_LOGICAL_CHARS,
    DEFAULT_ESCAPEABLE_CHARS,
    DEFAULT_PARSE_TERMINATE_CHARS,
    DEFAULT_FIRST_ARG_IS_BINARY,
};
