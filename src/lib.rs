//! Simple Command Line Argument Parser for Rust
//! 
//! A library which uses matchers to parse a command line
//! 
#![allow(clippy::collapsible_else_if)]

pub mod error;
pub mod regex_or_text;
pub mod matcher;
pub mod arg;
pub mod parser;

pub use error::{
    Error,
};

pub use regex_or_text::{
    RegexOrText,
};

pub use matcher:: {
    Matcher,
    Matchers,
    DefaultTagType,
    OptionHasValue,
    DEFAULT_OPTION_HAS_VALUE,
    OPTION_VALUE_CAN_START_WITH_ANNOUNCER_CHAR,
};

pub use arg::{
    ArgProperties,
    OptionProperties,
    ParamProperties,
    Arg,
    Args,
};

pub use parser::{
    Parser,
    EmbedQuoteCharMethod,
    DEFAULT_QUOTE_CHAR,
    DEFAULT_OPTION_ANNOUNCER_CHARS,
    DEFAULT_OPTION_CODES_CASE_SENSITIVE,
    DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
    DEFAULT_OPTION_VALUE_ANNOUNCER_CHARS,
    DEFAULT_OPTION_VALUES_CASE_SENSITIVE,
    DEFAULT_OPTION_VALUES_CAN_START_WITH_OPTION_ANNOUNCER_CHAR,
    DEFAULT_PARAMS_CASE_SENSITIVE,
    DEFAULT_PARAMS_CAN_START_WITH_OPTION_ANNOUNCER_CHAR,
    DEFAULT_EMBED_QUOTE_CHAR_METHOD,
    DEFAULT_ESCAPE_CHAR,
    DEFAULT_PARSE_TERMINATE_CHARS,
};
