use crate::regex_or_text::{RegexOrText};

/// The type that arguments will be tagged with if a [Parser](crate::Parser) is not declared with a `O` or `P` generic parameter.
pub type DefaultTagType = usize;

/// Specifies how a matcher determines whether an option includes a value.
#[derive(Debug, PartialEq, Eq)]
pub enum OptionHasValue {
    /// Option must always include a value.
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
pub enum MatchArgTypeId {
    /// Argument must be an Option
    Option,
    /// Argument must be a Parameter
    Param,
}

/// Default value of assigned to [Matcher::option_has_value] when a new [Matcher](Matcher) is created.
pub const DEFAULT_OPTION_HAS_VALUE: OptionHasValue = OptionHasValue::Never;

/// Contains a set of filters which can be used to match one or more arguments in a command line. An argument must meet all filter
/// conditions in a matcher for a match to occur.
/// 
/// The following filters are supported:
/// * Index of an argument: [arg_indices](Self::arg_indices)
/// * Type of argument: [arg_type](Self::arg_type)
/// * Index of a parameter argument: [param_indices](Self::param_indices)
/// * Parameter text: [value_text](Self::value_text)
/// * Index of an option argument: [option_indices](Self::option_indices)
/// * Option code: [option_codes](Self::option_codes)
/// * Whether an option has a value: [option_has_value](Self::option_has_value) 
/// * Whether an option value can start with an [option announcer character](crate::Parser::option_announcer_chars): 
/// [option_value_can_start_with_option_announcer](Self::option_value_can_start_with_option_announcer)
/// * Option value text: [value_text](Self::value_text)
/// 
/// Before parsing a command line, the [Parser](crate::Parser) instance should be assigned one or more `Matcher` instance. When a command line
/// is parsed, all arguments must be matched by a matcher. If zero matchers are assigned to a Parser, then all arguments will be matched.
/// 
/// It is possible for an argument to be matched by more than one matcher. For each argument, the parser will attempt to match a matcher
/// in the order of [Parser.matchers](crate::Parser::matchers). If a one matcher is more specific than another, insert it into this vector before
/// the other.
/// 
/// Each argument produced by the Parser's parse functions ([parse_line](crate::Parser::parse_line), [parse_env](crate::Parser::parse_env),
/// [parse_env_args](crate::Parser::parse_env_args)), will include a reference to the matcher which matched it. The properties
/// of a matcher can be used by the application when processing an argument. The [Matcher.option_tag](Self::option_tag) and
/// [Matcher.param_tag](Self::param_tag) can be assigned enums to assist with this processing. These enums can be used in match arms to easily
/// identify arguments.
#[derive(Debug)]
pub struct Matcher<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    index: usize,
    name: String,
    help: Option<String>,
    option_tag: O,
    param_tag: P,
    arg_indices: Option<Vec<usize>>,
    arg_type: Option<MatchArgTypeId>,
    option_indices: Option<Vec<usize>>,
    option_codes: Option<Vec<RegexOrText>>,
    option_has_value: OptionHasValue,
    option_value_can_start_with_option_announcer: bool,
    param_indices: Option<Vec<usize>>,
    /// Optionally specifies the text an option value or parameter needs to be equal to.
    value_text: Option<RegexOrText>,
}

impl<O: Default, P: Default> Matcher<O, P> {
    /// Create a new matcher with the specified name.
    pub fn new(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            ..Default::default()
        }
    }

    /// Create a new matcher with the specified name which matches option arguments.
    pub fn new_option(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            arg_type: Some(MatchArgTypeId::Option),
            ..Default::default()
        }
    }

    /// Create a new matcher with the specified name which matches parameter arguments.
    pub fn new_param(name: &str) -> Self {
        Matcher {
            name: String::from(name),
            arg_type: Some(MatchArgTypeId::Param),
            ..Default::default()
        }
    }

    /// Get the name of a matcher.
    /// 
    /// This is used for display and help purposes.  It is not used as a match filter.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name of a matcher.
    pub fn set_name(&mut self, value: &str) -> &mut Self {
        self.name = String::from(value);
        self
    }

    /// Get the index of the matcher within the Parser's [matchers vector](crate::Parser::matchers).
    pub fn index(&self) -> usize {
        self.index
    }

    pub (crate) fn set_index(&mut self, value: usize) {
        self.index = value;
    }

    /// Get the help text associated with a matcher
    pub fn help(&self) -> &Option<String> {
        &self.help
    }

    /// Set the help text associated with a matcher as an Option
    pub fn set_help(&mut self, value: Option<String>) -> &mut Self {
        self.help = value;
        self
    }

    /// Set the help text associated with a matcher
    pub fn some_help(&mut self, value: &str) -> &mut Self {
        self.help = Some(String::from(value));
        self
    }

    /// Clear the help text associated with a matcher
    pub fn none_help(&mut self) -> &mut Self {
        self.help = None;
        self
    }

    /// A value that can be used to identify option arguments matched by a matcher. Is of the type specified by the
    /// [Parser's](crate::Parser) `O` generic parameter. Normally `O` is an enum type which can be used in match statements
    /// which process the array of [Arg](crate::Arg) variants returned by a parse function.
    pub fn option_tag(&self) -> &O {
        &self.option_tag
    }

    /// Set the option tag value. See [option_tag](Self::option_tag).
    pub fn set_option_tag(&mut self, value: O) -> &mut Self {
        self.option_tag = value;
        self
    }

    /// A value that can be used to identify parameter arguments matched by a matcher. Is of the type specified by the
    /// [Parser's](crate::Parser) `P` generic parameter. Normally `P` is an enum type which can be used in match statements
    /// which process the array of [Arg](crate::Arg) variants returned by a parse function.
    pub fn param_tag(&self) -> &P {
        &self.param_tag
    }

    /// Set the parameter tag value. See [param_tag](Self::param_tag).
    pub fn set_param_tag(&mut self, value: P) -> &mut Self {
        self.param_tag = value;
        self
    }

    /// Match Filter: If `None`, all arguments accepted. Otherwise accepts an argument whose index is included in the vector.
    pub fn arg_indices(&self) -> &Option<Vec<usize>> {
        &self.arg_indices
    }

    /// Match Filter: Get the [arg_indices](Self::arg_indices) as a slice.
    pub fn arg_indices_as_slice(&self) -> &[usize] {
        if let Some(arg_indices) = self.arg_indices.as_ref() {
            arg_indices
        } else {
            &[]
        }
    }

    /// Match Filter: Set [arg_indices](Self::arg_indices).
    pub fn set_arg_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.arg_indices = value;
        self
    }

    /// Match Filter: Set [arg_indices](Self::arg_indices) from an array of `usize`.
    pub fn some_arg_indices(&mut self, value: &[usize]) -> &mut Self {
        self.arg_indices = Some(Vec::from(value));
        self
    }

    /// Match Filter: Set [arg_indices](Self::arg_indices) to `None` so that it accepts all arguments.
    pub fn none_arg_indices(&mut self) -> &mut Self {
        self.arg_indices = None;
        self
    }

    /// Match Filter: If `None`, accepts all arguments. Otherwise accepts arguments of the [type](MatchArgTypeId) specified.
    pub fn arg_type(&self) -> &Option<MatchArgTypeId> {
        &self.arg_type
    }

    /// Match Filter: Set [arg_type](Self::arg_type).
    pub fn set_arg_type(&mut self, value: Option<MatchArgTypeId>) -> &mut Self {
        self.arg_type = value;
        self
    }

    /// Match Filter: Set the [arg_type](Self::arg_type) to a [type](MatchArgTypeId) value.
    pub fn some_arg_type(&mut self, value: MatchArgTypeId) -> &mut Self {
        self.arg_type = Some(value);
        self
    }

    /// Match Filter: Set [arg_type](Self::arg_type) to `None` so that it accepts all arguments.
    pub fn none_arg_type(&mut self) -> &mut Self {
        self.arg_type = None;
        self
    }

    /// Match Filter: If `None`, all arguments accepted. Otherwise accepts a parameter argument whose index is included
    /// in the vector.
    pub fn param_indices(&self) -> &Option<Vec<usize>> {
        &self.param_indices
    }

    /// Match Filter: Get the [param_indices](Self::param_indices) as a slice.
    pub fn param_indices_as_slice(&self) -> &[usize] {
        if let Some(param_indices) = self.param_indices.as_ref() {
            param_indices
        } else {
            &[]
        }
    }

    /// Match Filter: Set [param_indices](Self::param_indices).
    pub fn set_param_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.param_indices = value;
        self
    }

    /// Match Filter: Set [param_indices](Self::param_indices) from an array of `usize`.
    pub fn some_param_indices(&mut self, value: &[usize]) -> &mut Self {
        self.param_indices = Some(Vec::from(value));
        self
    }

    /// Match Filter: Set [param_indices](Self::param_indices) to `None` so that it accepts all arguments.
    pub fn none_param_indices(&mut self) -> &mut Self {
        self.param_indices = None;
        self
    }

    /// Match Filter: If `None`, all arguments accepted. Otherwise accepts an option argument whose index is included in the vector.
    pub fn option_indices(&self) -> &Option<Vec<usize>> {
        &self.option_indices
    }

    /// Match Filter: Get the [option_indices](Self::option_indices) as a slice.
    pub fn option_indices_as_slice(&self) -> &[usize] {
        if let Some(option_indices) = self.option_indices.as_ref() {
            option_indices
        } else {
            &[]
        }
    }

    /// Match Filter: Set [option_indices](Self::option_indices).
    pub fn set_option_indices(&mut self, value: Option<Vec<usize>>) -> &mut Self {
        self.option_indices = value;
        self
    }

    /// Match Filter: Set [option_indices](Self::option_indices) from an array of `usize`.
    pub fn some_option_indices(&mut self, value: &[usize]) -> &mut Self {
        self.option_indices = Some(Vec::from(value));
        self
    }

    /// Match Filter: Set [option_indices](Self::option_indices) to `None` so that it accepts all arguments.
    pub fn none_option_indices(&mut self) -> &mut Self {
        self.option_indices = None;
        self
    }

    /// Match Filter: If `None`, all arguments accepted.  Otherwise accepts an option argument whose code is matched by any of the
    /// [RegexOrText](RegexOrText) in the vector.
    pub fn option_codes(&self) -> &Option<Vec<RegexOrText>> {
        &self.option_codes
    }

    /// Match Filter: Get the [option_codes](Self::option_codes) [RegexOrTexts](RegexOrText) as a slice.
    pub fn option_codes_as_slice(&self) -> &[RegexOrText] {
        if let Some(option_codes) = self.option_codes.as_ref() {
            option_codes
        } else {
            &[]
        }
    }

    /// Match Filter: Set [option_codes](Self::option_codes).
    pub fn set_option_codes(&mut self, value: Option<Vec<RegexOrText>>) -> &mut Self {
        self.option_codes = value;
        self
    }

    /// Match Filter: Set [option_codes](Self::option_codes) from an array of [RegexOrText](RegexOrText).
    pub fn some_option_codes(&mut self, value: &[RegexOrText]) -> &mut Self {
        self.option_codes = Some(Vec::from(value));
        self
    }

    /// Match Filter: Set [option_codes](Self::option_codes) to `None` so that it accepts all arguments.
    pub fn none_option_codes(&mut self) -> &mut Self {
        self.option_codes = None;
        self
    }

    /// Match Filter: Accepts an option argument with or without an option value according to the
    /// [OptionHasValue](OptionHasValue) variant. (Default: [Never](OptionHasValue::Never))
    pub fn option_has_value(&self) -> &OptionHasValue {
        &self.option_has_value
    }

    /// Match Filter: Set [option_has_value](Self::option_has_value).
    pub fn set_option_has_value(&mut self, value: OptionHasValue) -> &mut Self {
        self.option_has_value = value;
        self
    }

    /// Match Filter: Specifies whether an option argument with a value which starts with an
    /// [option announcer character](crate::Parser::option_announcer_chars) is accepted.
    /// 
    /// Option values beginning with an option announcer character can be both confusing and convenient. If they can start
    /// with option announcers, they may easily be confused with being a separate option.  However they can be convenient,
    /// for example, if the option value specifies numbers that can be negative (assuming '-' is the option announcer).
    /// Whether to allow option values to begin with an option announcer will depend on the context.
    /// 
    /// To start an option value with an option announcer when this is not allowed, the value should either
    /// be enclosed in quotes or the announcer should be escaped.
    /// 
    /// Currently starting an option value with an option announcer when [option_has_value](Matcher::option_has_value) is
    /// [IfPossible](OptionHasValue::IfPossible) is not supported.
    pub fn option_value_can_start_with_option_announcer(&self) -> bool {
        self.option_value_can_start_with_option_announcer
    }

    /// Match Filter: Set [option_value_can_start_with_option_announcer](Self::option_value_can_start_with_option_announcer).
    pub fn set_option_value_can_start_with_option_announcer(&mut self, value: bool) -> &mut Self {
        self.option_value_can_start_with_option_announcer = value;
        self
    }

    /// Match Filter: If `None`, all arguments accepted.  Otherwise, if argument is a parameter, then the argument will be accepted
    /// if the parameter's text is matched by the [RegexOrText](RegexOrText). If the argument is an option and has a value, then the
    /// argument is accepted if the option value text is matched by the [RegexOrText](RegexOrText). In all other cases, the
    /// argument is accepted.
    pub fn value_text(&self) -> &Option<RegexOrText> {
        &self.value_text
    }

    /// Match Filter: Set [value_text](Self::value_text).
    pub fn set_value_text(&mut self, value: Option<RegexOrText>) -> &Self {
        self.value_text = value;
        self
    }

    /// Match Filter: Set [value_text](Self::value_text) to a [RegexOrText](RegexOrText).
    pub fn some_value_text(&mut self, value: RegexOrText) -> &Self {
        self.value_text = Some(value);
        self
    }

    /// Match Filter: Set [value_text](Self::value_text) to `None` so that it accepts all arguments.
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
            option_has_value: DEFAULT_OPTION_HAS_VALUE,
            option_value_can_start_with_option_announcer: false,
            param_indices: None,
            value_text: None
        }
    }
}

/// A vector of [Matchers](Matcher)
pub type Matchers<O, P> = Vec<Matcher<O, P>>;
