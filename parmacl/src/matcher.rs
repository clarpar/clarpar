use crate::regex_or_text::{RegexOrText};

pub type DefaultTagType = usize;

/// Specifies how a matcher determines whether an option includes a value.
#[derive(Debug, PartialEq, Eq)]
pub enum OptionHasValue {
    /// Option must always include a value
    Always,
    /// If it is possible for the next argument after the option code to be its value, then
    /// the matcher treats the option as having a value.  Otherwise it is treated as not
    /// having a value.
    /// 
    /// This is only possible when a whitespace [option value announcer](crate::Parser::option_value_announcer_chars) is
    /// used.
    /// 
    /// Currently, if an option value which begins with an option announcer, it cannot be a possible value for an
    /// option when `IfPossible` is specified. That is,
    /// [Matcher.option_value_can_start_with_option_announcer](Matcher::option_value_can_start_with_option_announcer) is
    /// ignored when `IfPossible` is specified.
    IfPossible,
    /// Option never has a value. It is used as a flag.
    Never,
}

/// Specifies whether an argument is an option or a parameter.
#[derive(Debug, PartialEq, Eq)]
pub enum MatchArgType {
    Option,
    Param,
}

pub const DEFAULT_OPTION_HAS_VALUE: OptionHasValue = OptionHasValue::Never;

/// Contains a set of filters which can be used to match one or more arguments in a command line. An argument must match all filters in
/// a matcher for a match to occur.
/// 
/// Before parsing a command line, the `[Parser](Parser)` instance should be assigned one or more `Matcher` instance. When a command line
/// is parsed, all arguments must be matched by a matcher.
/// 
/// It is possible for an argument to be matched by more than one matcher. For each argument, the parser will attempt to match a matcher
/// in the order of `[Parser.matchers](Parmacl::matchers)`. If a one matcher is more specific than another, insert it into this vector before
/// the other.
/// 
/// Each argument produced by the `[Parser.parse](Parser::parse)` function, will include a reference to the matcher which matched it. The properties
/// of a matcher can be used by the application when processing an argument. The `[Matcher.option_tag](Matcher::option_tag)` and
/// `[Matcher.param_tag](Matcher::param_tag)` can be assigned enums to assist with this processing. These enums can be used in match arms to easily
/// identify arguments.
#[derive(Debug)]
pub struct Matcher<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    /// The index of the matcher in the parser matchers vector.
    index: usize,
    /// The name of the matcher.  This is only used for display purposes
    name: String,
    /// A help string associated with the matcher.  This is only used for display purposes
    help: Option<String>,
    /// Can be assigned an arbitrary type. Normally an enum is assigned which assists with matching any resultant option arguments.
    option_tag: O,
    /// Can be assigned an arbitrary type. Normally an enum is assigned which assists with matching any resultant parameter arguments.
    param_tag: P,
    // filters
    /// Optionally specifies the argument indices one of which an argument needs to be at.
    arg_indices: Option<Vec<usize>>,
    /// Optionally specifies an argument is an option or parameter.
    arg_type: Option<MatchArgType>,
    /// Optionally specifies the option indices one of which an option needs to be at.
    option_indices: Option<Vec<usize>>,
    /// Optionally specifies the possible codes one of which an option needs to be equal to.
    option_codes: Option<Vec<RegexOrText>>,
    /// Optionally specifies whether this option includes a value.
    option_has_value: Option<OptionHasValue>,
    /// Specifies whether the first character of an option value can be the option announcer character. (Default: false)
    /// 
    /// Option values beginning with an option announcer character can be both confusing and convenient. If they can start
    /// with option announcers, they may easily be confused with being a separate option.  However they can be convenient,
    /// for example, if the option value specifies numbers that can be negative  (assuming '-' is the option announcer).
    /// Whether to allow option values to begin with an option announcer will depend on the context.
    /// 
    /// To start an option value with an option announcer when this is not allowed, the value should either
    /// be enclosed in quotes or the announcer should be escaped.
    /// 
    /// Currently starting an option value with an option announcer when [option_has_value](Matcher::option_has_value) is
    /// [IfPossible](OptionHasValue::IfPossible) is not supported.
    option_value_can_start_with_option_announcer: bool,
    /// Optionally specifies the parameter indices one of which a parameter needs to be at.
    param_indices: Option<Vec<usize>>,
    /// Optionally specifies the text an option value or parameter needs to be equal to.
    value_text: Option<RegexOrText>,
}

impl<O: Default, P: Default> Matcher<O, P> {
    pub fn new(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub (crate) fn set_index(&mut self, value: usize) {
        self.index = value;
    }

    pub fn new_option(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            arg_type: Some(MatchArgType::Option),
            ..Default::default()
        }
    }

    pub fn new_param(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            arg_type: Some(MatchArgType::Param),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) -> &mut Self {
        self.name = String::from(value);
        self
    }

    pub fn help(&self) -> &Option<String> {
        &self.help
    }

    pub fn set_help(&mut self, value: Option<String>) -> &mut Self {
        self.help = value;
        self
    }

    pub fn some_help(&mut self, value: &str) -> &mut Self {
        self.help = Some(String::from(value));
        self
    }

    pub fn none_help(&mut self) -> &mut Self {
        self.help = None;
        self
    }

    pub fn option_tag(&self) -> &O {
        &self.option_tag
    }

    pub fn set_option_tag(&mut self, value: O) -> &mut Self {
        self.option_tag = value;
        self
    }

    pub fn param_tag(&self) -> &P {
        &self.param_tag
    }

    pub fn set_param_tag(&mut self, value: P) -> &mut Self {
        self.param_tag = value;
        self
    }

    pub fn arg_indices(&self) -> &Option<Vec<usize>> {
        &self.arg_indices
    }

    pub fn arg_indices_as_slice(&self) -> &[usize] {
        if let Some(arg_indices) = self.arg_indices.as_ref() {
            arg_indices
        } else {
            &[]
        }
    }

    pub fn set_arg_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.arg_indices = value;
        self
    }

    pub fn some_arg_indices(&mut self, value: &[usize]) -> &mut Self {
        self.arg_indices = Some(Vec::from(value));
        self
    }

    pub fn none_arg_indices(&mut self) -> &mut Self {
        self.arg_indices = None;
        self
    }

    pub fn arg_type(&self) -> &Option<MatchArgType> {
        &self.arg_type
    }

    pub fn set_arg_type(&mut self, value: Option<MatchArgType>) -> &mut Self {
        self.arg_type = value;
        self
    }

    pub fn some_arg_type(&mut self, value: MatchArgType) -> &mut Self {
        self.arg_type = Some(value);
        self
    }

    pub fn none_arg_type(&mut self) -> &mut Self {
        self.arg_type = None;
        self
    }

    pub fn option_indices(&self) -> &Option<Vec<usize>> {
        &self.option_indices
    }

    pub fn option_indices_as_slice(&self) -> &[usize] {
        if let Some(option_indices) = self.option_indices.as_ref() {
            option_indices
        } else {
            &[]
        }
    }

    pub fn set_option_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.option_indices = value;
        self
    }

    pub fn some_option_indices(&mut self, value: &[usize]) -> &mut Self {
        self.option_indices = Some(Vec::from(value));
        self
    }

    pub fn none_option_indices(&mut self) -> &mut Self {
        self.option_indices = None;
        self
    }

    pub fn option_codes(&self) -> &Option<Vec<RegexOrText>> {
        &self.option_codes
    }

    pub fn option_codes_as_slice(&self) -> &[RegexOrText] {
        if let Some(option_codes) = self.option_codes.as_ref() {
            option_codes
        } else {
            &[]
        }
    }

    pub fn set_option_codes(&mut self, value: Option<Vec<RegexOrText>>) -> &mut Self {
        self.option_codes = value;
        self
    }

    pub fn some_option_codes(&mut self, value: &[RegexOrText]) -> &mut Self {
        self.option_codes = Some(Vec::from(value));
        self
    }

    pub fn none_option_codes(&mut self) -> &mut Self {
        self.option_codes = None;
        self
    }

    pub fn option_has_value(&self) -> &Option<OptionHasValue> {
        &self.option_has_value
    }

    pub fn set_option_has_value(&mut self, value: Option<OptionHasValue>) -> &mut Self {
        self.option_has_value = value;
        self
    }

    pub fn some_option_has_value(&mut self, value: OptionHasValue) -> &mut Self {
        self.option_has_value = Some(value);
        self
    }

    pub fn none_option_has_value(&mut self) -> &mut Self {
        self.option_has_value = None;
        self
    }

    pub fn option_value_can_start_with_option_announcer(&self) -> bool {
        self.option_value_can_start_with_option_announcer
    }

    pub fn set_option_value_can_start_with_option_announcer(&mut self, value: bool) -> &mut Self {
        self.option_value_can_start_with_option_announcer = value;
        self
    }

    pub fn param_indices(&self) -> &Option<Vec<usize>> {
        &self.param_indices
    }

    pub fn param_indices_as_slice(&self) -> &[usize] {
        if let Some(param_indices) = self.param_indices.as_ref() {
            param_indices
        } else {
            &[]
        }
    }

    pub fn set_param_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.param_indices = value;
        self
    }

    pub fn some_param_indices(&mut self, value: &[usize]) -> &mut Self {
        self.param_indices = Some(Vec::from(value));
        self
    }

    pub fn none_param_indices(&mut self) -> &mut Self {
        self.param_indices = None;
        self
    }

    pub fn value_text(&self) -> &Option<RegexOrText> {
        &self.value_text
    }

    pub fn set_value_text(&mut self, value: Option<RegexOrText>) -> &Self {
        self.value_text = value;
        self
    }

    pub fn some_value_text(&mut self, value: RegexOrText) -> &Self {
        self.value_text = Some(value);
        self
    }

    pub fn none_value_text(&mut self) -> &Self {
        self.value_text = None;
        self
    }
}

impl<O: Default, P: Default> Default for Matcher<O, P> {
    fn default() -> Self {
        Matcher {
            name: String::from(""),
            index: 0,
            help: None,
            option_tag: O::default(),
            param_tag: P::default(),
            arg_indices: None,
            arg_type: None,
            option_indices: None,
            option_codes: None,
            option_has_value: None,
            option_value_can_start_with_option_announcer: false,
            param_indices: None,
            value_text: None
        }
    }
}

pub type Matchers<O, P> = Vec<Matcher<O, P>>;
