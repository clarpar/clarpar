use crate::regex_or_text::{RegexOrText};

pub type DefaultTagType = usize;

/// Specifies how a matcher determines whether an Option includes a value
pub enum OptionHasValue {
    /// Option must always include a value
    Always,
    /// Option never has a value
    Never,
    /// If the option must have a value (for example, its code is followed by a non whitespace option value
    /// announcer), then the matcher treats the option as having a value.  Otherwise, the option is treated
    /// as not having a value (even if the following argument could be its value).
    OnlyIfMust,
    /// If it is possible for the next argument after the option code to be its value, then
    /// the matcher treats the option as have a value.  Otherwise it is treated as not
    /// having a value.
    IfPossible,
}

pub const DEFAULT_OPTION_HAS_VALUE: OptionHasValue = OptionHasValue::OnlyIfMust;
pub const OPTION_VALUE_CAN_START_WITH_ANNOUNCER_CHAR: bool = false;

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
    /// specifies whether this option includes a value
    pub option_has_value: Option<OptionHasValue>,
    pub option_value_can_start_with_announcer_char: Option<bool>,
    pub param_indices: Option<Vec<usize>>,
    pub value_text: Option<RegexOrText>,
}

impl<O, P> Matcher<O, P> {
    pub fn new(name: String) -> Self {
        Matcher {
            name,
            ..Default::default()
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
            option_value_can_start_with_announcer_char: None,
            param_indices: None,
            value_text: None
        }
    }
}

pub type Matchers<O, P> = Vec<Matcher<O, P>>;
