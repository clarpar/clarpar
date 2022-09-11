use crate::matcher::Matcher;

/// Trait with getters for properties common to all [Arg enum](Arg) variant properties.
pub trait ArgProperties<O: Default, P: Default> {
    /// Matcher which matched this argument.
    fn matcher(&self) -> &Matcher<O, P>;
    /// Index of character where the parsed argument started.  If a line was parsed, this will be the index in the line.
    /// If environmental arguments were parsed, this will be the index in the environmental argument where the
    /// parsed argument started.
    fn char_index(&self) -> usize;
    /// Approximate index of character in the line where the parsed argument started. If a line was parsed, this will
    /// be the same as [char_index](Self::char_index) and be exact. If environmental arguments were parsed, this will
    /// an approximate position in the line.
    fn env_line_approximate_char_index(&self) -> usize;
    /// Index of the parsed argument.
    fn arg_index(&self) -> usize;
    /// Index of the environment argument in which the parsed argument started.  If a line is parsed, this will always
    /// be zero.
    fn env_arg_index(&self) -> usize;
}

/// Properties for an [Option Arg variant](Arg::Option)
#[derive(Debug)]
pub struct OptionProperties<'a, O: Default, P: Default> {
    /// Matcher which matched this argument.
    pub matcher: &'a Matcher<O, P>,
    /// Index of character where the parsed argument started.  If a line was parsed, this will be the index in the line.
    /// If environmental arguments were parsed, this will be the index in the environmental argument where the
    /// parsed argument started.
    pub char_index: usize,
    /// Approximate index of character in the line where the parsed argument started. If a line was parsed, this will
    /// be the same as [char_index](Self::char_index) and be exact. If environmental arguments were parsed, this will
    /// an approximate position in the line.
    pub env_line_approximate_char_index: usize,
    /// Index of the parsed argument.
    pub arg_index: usize,
    /// Index of the environment argument in which the parsed argument started.  If a line is parsed, this will always
    /// be zero.
    pub env_arg_index: usize,
    /// Index of the parsed option arguments. (Parameters arguments are ignored in this index.)
    pub option_index: usize,
    /// Option code of the option argument.
    pub code: String,
    /// Text in the option value. If option did not have a value, then this holds `None`.
    pub value_text: Option<String>,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for OptionProperties<'a, O, P> {
    fn matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn char_index(&self) -> usize {
        self.char_index
    }
    fn env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn arg_index(&self) -> usize {
        self.arg_index
    }
    fn env_arg_index(&self) -> usize {
        self.env_arg_index
    }
}

/// Properties for an [Param Arg variant](Arg::Param)
#[derive(Debug)]
pub struct ParamProperties<'a, O: Default, P: Default> {
    /// Matcher which matched this argument.
    pub matcher: &'a Matcher<O, P>,
    /// Index of character where the argument started.  If a line is parsed, this will be the index in the line.
    /// If environmental arguments are parsed, this will be the index in the environmental argument where the
    /// parsed argument started.
    pub char_index: usize,
    /// Approximate index of character in the line where the parsed argument started. If a line was parsed, this will
    /// be the same as [char_index](Self::char_index) and be exact. If environmental arguments were parsed, this will
    /// an approximate position in the line.
    pub env_line_approximate_char_index: usize,
    /// Index of the parsed argument.
    pub arg_index: usize,
    /// Index of the environment argument in which the parsed argument started.  If a line is parsed, this will always
    /// be zero.
    pub env_arg_index: usize,
    /// Index of the parsed parameter arguments. (Option arguments are ignored in this index.)
    pub param_index: usize,
    /// Text in the parameter.
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for ParamProperties<'a, O, P> {
    fn matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn char_index(&self) -> usize {
        self.char_index
    }
    fn env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn arg_index(&self) -> usize {
        self.arg_index
    }
    fn env_arg_index(&self) -> usize {
        self.env_arg_index
    }
}

/// Properties for an [Binary Arg variant](Arg::Binary)
#[derive(Debug)]
pub struct BinaryProperties<'a, O: Default, P: Default> {
    /// An internal matcher used to match binary arguments.
    pub matcher: &'a Matcher<O, P>,
    /// Index of character where the argument started.  If a line is parsed, this will be the index in the line.
    /// If environmental arguments are parsed, this will be the index in the environmental argument where the
    /// parsed argument started. Normally 0.
    pub char_index: usize,
    /// Approximate index of character in the line where the parsed argument started. If a line was parsed, this will
    /// be the same as [char_index](Self::char_index) and be exact. If environmental arguments were parsed, this will
    /// an approximate position in the line. Normally 0.
    pub env_line_approximate_char_index: usize,
    /// Index of the parsed argument. Always 0.
    pub arg_index: usize,
    /// Index of the environment argument in which the parsed argument started.  Normally 0.
    pub env_arg_index: usize,
    /// Text normally specifying the binary name or path.
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for BinaryProperties<'a, O, P> {
    fn matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn char_index(&self) -> usize {
        self.char_index
    }
    fn env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn arg_index(&self) -> usize {
        self.arg_index
    }
    fn env_arg_index(&self) -> usize {
        self.env_arg_index
    }
}

/// An enum with variants for the 3 different types of parsed arguments. Each variant has an associated
/// struct which holds the properties for that type of argument.
/// 
/// The [Parser](crate::Parser)'s parse functions will return an array of these variants, one for each
/// argument parsed, if the parse operation was successful.
#[derive(Debug)]
pub enum Arg<'a, O: Default, P: Default> {
    /// The first argument which normally holds the binary/executable's path or name.
    Binary(BinaryProperties<'a, O, P>),
    /// A parameter parsed argument.
    Param(ParamProperties<'a, O, P>),
    /// An option parsed argument.
    Option(OptionProperties<'a, O, P>),
}

/// Vector of [Arg](Arg) enum variants.
pub type Args<'a, O, P> = Vec<Arg<'a, O, P>>;
