use crate::matcher::Matcher;

pub trait ArgProperties<O: Default, P: Default> {
    fn get_matcher(&self) -> &Matcher<O, P>;
    fn get_line_char_index(&self) -> usize;
    fn get_arg_index(&self) -> usize;
}

#[derive(Debug)]
pub struct OptionProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub option_index: usize,
    pub code: String,
    pub value_text: Option<String>,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for OptionProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_line_char_index(&self) -> usize {
        self.line_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
}

#[derive(Debug)]
pub struct ParamProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub param_index: usize,
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for ParamProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_line_char_index(&self) -> usize {
        self.line_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
}

#[derive(Debug)]
pub struct BinaryProperties<'a, O: Default, P: Default> {
    pub matcher: &'a Matcher<O, P>,
    pub line_char_index: usize,
    pub arg_index: usize,
    pub value_text: String,
}

impl<'a, O: Default, P: Default> ArgProperties<O, P> for BinaryProperties<'a, O, P> {
    fn get_matcher(&self) -> &Matcher<O, P> {
        self.matcher
    }
    fn get_line_char_index(&self) -> usize {
        self.line_char_index
    }
    fn get_arg_index(&self) -> usize {
        self.arg_index
    }
}

#[derive(Debug)]
pub enum Arg<'a, O: Default, P: Default> {
    Binary(BinaryProperties<'a, O, P>),
    Param(ParamProperties<'a, O, P>),
    Option(OptionProperties<'a, O, P>),
}

pub type Args<'a, O, P> = Vec<Arg<'a, O, P>>;
