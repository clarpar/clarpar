use crate::regex_or_text::{RegexOrText};

pub type DefaultTagType = usize;

/// Specifies how a matcher determines whether an option includes a value.
#[derive(PartialEq, Eq)]
pub enum OptionHasValue {
    /// Option must always include a value. The value must not start with an option announcer (normally `-`).
    /// 
    /// By not allowing values to begin with an option announcer, it prevents users from confusing option values
    /// with other options.  To include an option value that begins with an option announcer, the value should either
    /// be enclosed in quotes or the announcer should be escaped.
    AlwaysButValueMustNotStartWithOptionAnnouncer,
    /// Option must always include a value. The value may start with an option announcer (normally `-`).
    /// 
    /// By allowing values to start with an announcer, it allows option values that are numbers which can be negative,
    /// to be easily specified by users. (They do not have to be enclosed in quotes.)  However users should be clearly
    /// able to distinguish between such values and other option codes.
    AlwaysAndValueCanStartWithOptionAnnouncer,
    /// If it is possible for the next argument after the option code to be its value, then
    /// the matcher treats the option as have a value.  Otherwise it is treated as not
    /// having a value.
    /// 
    /// To prevent confusion, an option value which begins with an option announcer, cannot be a possible value for an
    /// option.
    IfPossible,
    /// Option never has a value.
    Never,
}

/// Specifies whether an argument is an option or a parameter.
#[derive(PartialEq, Eq)]
pub enum OptionOrParam {
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
pub struct Matcher<O: Default = DefaultTagType, P: Default = DefaultTagType> {
    /// The name of the matcher.  This is only used for display purposes
    pub name: String,
    /// A help string associated with the matcher.  This is only used for display purposes
    pub help: Option<String>,
    /// Can be assigned an arbitrary type. Normally an enum is assigned which assists with matching any resultant option arguments.
    pub option_tag: O,
    /// Can be assigned an arbitrary type. Normally an enum is assigned which assists with matching any resultant parameter arguments.
    pub param_tag: P,
    // filters
    /// Optionally specifies the argument indices one of which an argument needs to be at.
    pub arg_indices: Option<Vec<usize>>,
    /// Optionally specifies an argument is an option or parameter.
    pub option_or_param: Option<OptionOrParam>,
    /// Optionally specifies the option indices one of which an option needs to be at.
    pub option_indices: Option<Vec<usize>>,
    /// Optionally specifies the possible codes one of which an option needs to be equal to.
    pub option_codes: Option<Vec<RegexOrText>>,
    /// Optionally specifies whether this option includes a value.
    pub option_has_value: Option<OptionHasValue>,
    /// Optionally specifies the parameter indices one of which a parameter needs to be at.
    pub param_indices: Option<Vec<usize>>,
    /// Optionally specifies the text an option value or parameter needs to be equal to.
    pub value_text: Option<RegexOrText>,
}

impl<O: Default, P: Default> Matcher<O, P> {
    pub fn new(name: String) -> Self {
        Matcher {
            name,
            ..Default::default()
        }
    }

    pub fn new_option(name: String) -> Self {
        Matcher {
            name,
            option_or_param: Some(OptionOrParam::Option),
            ..Default::default()
        }
    }

    pub fn new_param(name: String) -> Self {
        Matcher {
            name,
            option_or_param: Some(OptionOrParam::Param),
            ..Default::default()
        }
    }
}

impl<O: Default, P: Default> Default for Matcher<O, P> {
    fn default() -> Self {
        Matcher {
            name: String::from(""),
            help: None,
            option_tag: O::default(),
            param_tag: P::default(),
            arg_indices: None,
            option_or_param: None,
            option_indices: None,
            option_codes: None,
            option_has_value: None,
            param_indices: None,
            value_text: None
        }
    }
}

pub type Matchers<O, P> = Vec<Matcher<O, P>>;
