use parmacl::{Parser, Matcher, Arg, RegexOrText};
#[derive(Default)]
enum OptionEnum {
    #[default] A,
    B,
}
#[derive(Default)]
enum ParamEnum {
    #[default] Param1,
    Param2,
    Param3,
}

#[test]
fn no_matches() {
    const COMMAND_LINE: &str = "param1 param2 -a -b param3";
    let parser: Parser = Parser::new();
    let args = parser.parse(COMMAND_LINE).unwrap();

    assert_eq!(args.len(), 5);

    for arg in args {
        match arg {
            Arg::Option(properties) => {
                match properties.option_index {
                    0 => {
                        assert_eq!(properties.arg_index, 2);
                        assert_eq!(properties.code, "a");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 14);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 3);
                        assert_eq!(properties.code, "b");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    _ => {
                        panic!("Unexpected option");
                    },
                }
            }
            Arg::Param(properties) => {
                match properties.param_index {
                    0 => {
                        assert_eq!(properties.arg_index, 0);
                        assert_eq!(properties.value_text, "param1");
                        assert_eq!(properties.line_char_index, 0);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 1);
                        assert_eq!(properties.value_text, "param2");
                        assert_eq!(properties.line_char_index, 7);
                    },
                    2 => {
                        assert_eq!(properties.arg_index, 4);
                        assert_eq!(properties.value_text, "param3");
                        assert_eq!(properties.line_char_index, 20);
                    },
                    _ => {
                        panic!("Unexpected param");
                    },
                }
            }
        }
    }
}

#[test]
fn matches() {
    const COMMAND_LINE: &str = "param1 param2 -a -b param3";
    let mut parser: Parser<OptionEnum, ParamEnum> = Parser::new();

    let mut param_1_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new(String::from("param1"));
    param_1_matcher.param_tag = ParamEnum::Param1;
    param_1_matcher.param_indices = Some(Vec::from([0]));
    parser.add_matcher(param_1_matcher);

    let mut param_2_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new(String::from("param2"));
    param_2_matcher.param_tag = ParamEnum::Param2;
    param_2_matcher.param_indices = Some(Vec::from([1]));
    parser.add_matcher(param_2_matcher);

    let mut param_3_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new(String::from("param3"));
    param_3_matcher.param_tag = ParamEnum::Param3;
    param_3_matcher.param_indices = Some(Vec::from([2]));
    parser.add_matcher(param_3_matcher);

    let mut opt_a_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new(String::from("optionA"));
    opt_a_matcher.option_tag = OptionEnum::A;
    opt_a_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("a")]));
    parser.add_matcher(opt_a_matcher);

    let mut opt_b_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new(String::from("optionB"));
    opt_b_matcher.option_tag = OptionEnum::B;
    opt_b_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("b")]));
    parser.add_matcher(opt_b_matcher);

    let args = parser.parse(COMMAND_LINE).unwrap();

    assert_eq!(args.len(), 5);

    for arg in args {
        match arg {
            Arg::Option(properties) => {
                match properties.matcher.option_tag {
                    OptionEnum::A => {
                        assert_eq!(properties.arg_index, 2);
                        assert_eq!(properties.option_index, 0);
                        assert_eq!(properties.code, "a");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 14);
                    },
                    OptionEnum::B => {
                        assert_eq!(properties.arg_index, 3);
                        assert_eq!(properties.option_index, 1);
                        assert_eq!(properties.code, "b");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                }
            }
            Arg::Param(properties) => {
                match properties.matcher.param_tag {
                    ParamEnum::Param1 => {
                        assert_eq!(properties.arg_index, 0);
                        assert_eq!(properties.param_index, 0);
                        assert_eq!(properties.value_text, "param1");
                        assert_eq!(properties.line_char_index, 0);
                    },
                    ParamEnum::Param2 => {
                        assert_eq!(properties.arg_index, 1);
                        assert_eq!(properties.param_index, 1);
                        assert_eq!(properties.value_text, "param2");
                        assert_eq!(properties.line_char_index, 7);
                    },
                    ParamEnum::Param3 => {
                        assert_eq!(properties.arg_index, 4);
                        assert_eq!(properties.param_index, 2);
                        assert_eq!(properties.value_text, "param3");
                        assert_eq!(properties.line_char_index, 20);
                    },
                }
            }
        }
    }
}