pub(crate) enum EnvChar {
    Separator,
    Unicode(char),
}

impl EnvChar {
    pub fn try_get_unicode_non_whitespace(&self) -> Option<char> {
        match self {
            EnvChar::Separator => None,
            EnvChar::Unicode(char) => Some(*char),
        }
    }
}