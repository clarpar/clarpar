use crate::matcher::Matcher;

pub trait ArgProperties<O: Default, P: Default> {
    fn get_matcher(&self) -> &Matcher<O, P>;
    fn get_char_index(&self) -> usize;
    fn get_env_line_approximate_char_index(&self) -> usize;
    fn get_arg_index(&self) -> usize;
    fn get_env_arg_index(&self) -> usize;
}

/// Properties for an [Arg option variant](Arg::Option)
#[derive(Debug)]
pub struct OptionProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub char_index: usize,
    pub env_line_approximate_char_index: usize,
    pub arg_index: usize,
    pub env_arg_index: usize,
    pub option_index: usize,
    pub code: String,
    pub value_text: Option<String>,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for OptionProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_char_index(&self) -> usize {
        self.char_index
    }
    fn get_env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
    fn get_env_arg_index(&self) -> usize {
        self.env_arg_index
    }
}

#[derive(Debug)]
pub struct ParamProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub char_index: usize,
    pub env_line_approximate_char_index: usize,
    pub arg_index: usize,
    pub env_arg_index: usize,
    pub param_index: usize,
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for ParamProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_char_index(&self) -> usize {
        self.char_index
    }
    fn get_env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
    fn get_env_arg_index(&self) -> usize {
        self.env_arg_index
    }
}

#[derive(Debug)]
pub struct BinaryProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub char_index: usize,
    pub env_line_approximate_char_index: usize,
    pub arg_index: usize,
    pub env_arg_index: usize,
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for BinaryProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_char_index(&self) -> usize {
        self.char_index
    }
    fn get_env_line_approximate_char_index(&self) -> usize {
        self.env_line_approximate_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
    fn get_env_arg_index(&self) -> usize {
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

/// Vector of Arg enum variants.
pub type Args<'a, O, P> = Vec<Arg<'a, O, P>>;
