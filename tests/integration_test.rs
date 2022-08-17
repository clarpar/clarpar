use parmacl::{Parser, Matcher, Arg, RegexOrText, OptionHasValue};
#[derive(Default)]
enum OptionEnum {
    #[default] A,
    B,
    C,
    D,
    E,
    F,
    G,
    HH,
    II,
    JJ,
}
#[derive(Default)]
enum ParamEnum {
    #[default] Param1,
    Param2,
    Param3,
    Param4,
    Param5,
    Param6,
}

#[test]
fn no_matchers() {
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
fn basic_matchers() {
    const COMMAND_LINE: &str = "param1 param2 -a -b param3 -c valueC_1 \"param4\" \"param 5\" -d \"value D1\" -e -valueE1 -F \"-value F1\" -g optvalueG1 param6 -hh -ii valueh --JJ optValueJJ1";
    let mut parser: Parser<OptionEnum, ParamEnum> = Parser::new();

    let mut param_1_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param1"));
    param_1_matcher.param_tag = ParamEnum::Param1;
    param_1_matcher.param_indices = Some(Vec::from([0]));
    parser.add_matcher(param_1_matcher);

    let mut param_2_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param2"));
    param_2_matcher.param_tag = ParamEnum::Param2;
    param_2_matcher.param_indices = Some(Vec::from([1]));
    parser.add_matcher(param_2_matcher);

    let mut param_3_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param3"));
    param_3_matcher.param_tag = ParamEnum::Param3;
    param_3_matcher.param_indices = Some(Vec::from([2]));
    parser.add_matcher(param_3_matcher);

    let mut param_4_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param4"));
    param_4_matcher.param_tag = ParamEnum::Param4;
    param_4_matcher.param_indices = Some(Vec::from([3]));
    parser.add_matcher(param_4_matcher);

    let mut param_5_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param5"));
    param_5_matcher.param_tag = ParamEnum::Param5;
    param_5_matcher.param_indices = Some(Vec::from([4]));
    parser.add_matcher(param_5_matcher);

    let mut param_6_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param6"));
    param_6_matcher.param_tag = ParamEnum::Param6;
    param_6_matcher.param_indices = Some(Vec::from([5]));
    parser.add_matcher(param_6_matcher);

    let mut opt_a_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionA"));
    opt_a_matcher.option_tag = OptionEnum::A;
    opt_a_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("a")]));
    parser.add_matcher(opt_a_matcher);

    let mut opt_b_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionB"));
    opt_b_matcher.option_tag = OptionEnum::B;
    opt_b_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("b")]));
    parser.add_matcher(opt_b_matcher);

    let mut opt_c_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionC"));
    opt_c_matcher.option_tag = OptionEnum::C;
    opt_c_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("c")]));
    parser.add_matcher(opt_c_matcher);

    let mut opt_d_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionD"));
    opt_d_matcher.option_tag = OptionEnum::D;
    opt_d_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("d")]));
    opt_d_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_d_matcher);

    let mut opt_e_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionE"));
    opt_e_matcher.option_tag = OptionEnum::E;
    opt_e_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("E")]));
    opt_e_matcher.option_has_value = Some(OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer);
    parser.add_matcher(opt_e_matcher);

    let mut opt_f_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionF"));
    opt_f_matcher.option_tag = OptionEnum::F;
    opt_f_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("f")]));
    parser.add_matcher(opt_f_matcher);

    let mut opt_g_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionG"));
    opt_g_matcher.option_tag = OptionEnum::G;
    opt_g_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("g")]));
    parser.add_matcher(opt_g_matcher);

    let mut opt_hh_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionHH"));
    opt_hh_matcher.option_tag = OptionEnum::HH;
    opt_hh_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("HH")]));
    parser.add_matcher(opt_hh_matcher);

    let mut opt_ii_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionII"));
    opt_ii_matcher.option_tag = OptionEnum::II;
    opt_ii_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("II")]));
    parser.add_matcher(opt_ii_matcher);

    let mut opt_jj_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionJJ"));
    opt_jj_matcher.option_tag = OptionEnum::JJ;
    opt_jj_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("jj")]));
    parser.add_matcher(opt_jj_matcher);

    let args = parser.parse(COMMAND_LINE).unwrap();

    assert_eq!(args.len(), 16);

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
                    OptionEnum::C => {
                        assert_eq!(properties.arg_index, 6);
                        assert_eq!(properties.option_index, 2);
                        assert_eq!(properties.code, "c");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 27);
                    },
                    OptionEnum::D => {
                        assert_eq!(properties.arg_index, 9);
                        assert_eq!(properties.option_index, 3);
                        assert_eq!(properties.code, "d");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 58);
                    },
                    OptionEnum::E => {
                        assert_eq!(properties.arg_index, 10);
                        assert_eq!(properties.option_index, 4);
                        assert_eq!(properties.code, "e");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    OptionEnum::F => {
                        assert_eq!(properties.arg_index, 11);
                        assert_eq!(properties.option_index, 5);
                        assert_eq!(properties.code, "F");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    OptionEnum::G => {
                        assert_eq!(properties.arg_index, 12);
                        assert_eq!(properties.option_index, 6);
                        assert_eq!(properties.code, "G");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    OptionEnum::HH => {
                        assert_eq!(properties.arg_index, 14);
                        assert_eq!(properties.option_index, 7);
                        assert_eq!(properties.code, "HH");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    OptionEnum::II => {
                        assert_eq!(properties.arg_index, 15);
                        assert_eq!(properties.option_index, 8);
                        assert_eq!(properties.code, "ii");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 17);
                    },
                    OptionEnum::JJ => {
                        assert_eq!(properties.arg_index, 16);
                        assert_eq!(properties.option_index, 9);
                        assert_eq!(properties.code, "JJ");
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
                    ParamEnum::Param4 => {
                        assert_eq!(properties.arg_index, 7);
                        assert_eq!(properties.param_index, 3);
                        assert_eq!(properties.value_text, "param4");
                        assert_eq!(properties.line_char_index, 39);
                    },
                    ParamEnum::Param5 => {
                        assert_eq!(properties.arg_index, 8);
                        assert_eq!(properties.param_index, 4);
                        assert_eq!(properties.value_text, "param 5");
                        assert_eq!(properties.line_char_index, 48);
                    },
                    ParamEnum::Param6 => {
                        assert_eq!(properties.arg_index, 13);
                        assert_eq!(properties.param_index, 5);
                        assert_eq!(properties.value_text, "param6");
                        assert_eq!(properties.line_char_index, 20);
                    },
                }
            }
        }
    }
}