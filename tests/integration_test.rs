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
    Hh,
    Ii,
    Jj,
    Kkkk,
    Ll,
    M,
    N,
    O,
    P,
}
#[derive(Default)]
enum ParamEnum {
    #[default] Param1,
    Param2,
    Param3,
    Param4,
    Param5,
    Param6,
    Param7,
    Param8,
    Param9,
    Param10,
}

#[test]
fn no_matchers() {
    const COMMAND_LINE: &str = "binary param1 param2 -a -b param3";
    let parser: Parser = Parser::new();
    let args = parser.parse(COMMAND_LINE).unwrap();

    assert_eq!(args.len(), 6);

    for arg in args {
        match arg {
            Arg::Binary(properties) => {
                assert_eq!(properties.arg_index, 0);
                assert_eq!(properties.value_text, "binary");
                assert_eq!(properties.line_char_index, 0);
            },
            Arg::Option(properties) => {
                match properties.option_index {
                    0 => {
                        assert_eq!(properties.arg_index, 3);
                        assert_eq!(properties.code, "a");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 21);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 4);
                        assert_eq!(properties.code, "b");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 24);
                    },
                    _ => {
                        panic!("Unexpected option");
                    },
                }
            }
            Arg::Param(properties) => {
                match properties.param_index {
                    0 => {
                        assert_eq!(properties.arg_index, 1);
                        assert_eq!(properties.value_text, "param1");
                        assert_eq!(properties.line_char_index, 7);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 2);
                        assert_eq!(properties.value_text, "param2");
                        assert_eq!(properties.line_char_index, 14);
                    },
                    2 => {
                        assert_eq!(properties.arg_index, 5);
                        assert_eq!(properties.value_text, "param3");
                        assert_eq!(properties.line_char_index, 27);
                    },
                    _ => {
                        panic!("Unexpected param");
                    },
                }
            }
        }
    }
}

const BASIC_MATCHERS_COMMAND_LINE: &str = "\
\"binary name\" \
param1 \
param2 \
-a \
-b \
param3 \
-c valueC_1 \
\"param4\" \
\"param 5\" \
-d \"value D1\" \
-e -valueE1 \
-F \"-value F1\" \
-g optvalueG1 \
param6 \
-hh \
-ii valueII1 \
-JJ -optValueJJ1 \
-kkkk \"valueKKKK1\" \
-LL \"-optValueLL1\" \
-m \"opt\"\"valueM\" \
-n opt\"valueN \
-o opt\"valueO\" \
-p opt\"\"valueP \
\"par\"\"am7\" \
par\"am8 \
par\"am9\" \
par\"\"am10 \
";

#[test]
fn basic_matchers() {
    let mut parser: Parser<OptionEnum, ParamEnum> = Parser::new();
    let command_line = String::from(BASIC_MATCHERS_COMMAND_LINE);

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

    let mut param_7_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param7"));
    param_7_matcher.param_tag = ParamEnum::Param7;
    param_7_matcher.param_indices = Some(Vec::from([6]));
    parser.add_matcher(param_7_matcher);

    let mut param_8_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param8"));
    param_8_matcher.param_tag = ParamEnum::Param8;
    param_8_matcher.param_indices = Some(Vec::from([7]));
    parser.add_matcher(param_8_matcher);

    let mut param_9_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param9"));
    param_9_matcher.param_tag = ParamEnum::Param9;
    param_9_matcher.param_indices = Some(Vec::from([8]));
    parser.add_matcher(param_9_matcher);

    let mut param_10_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_param(String::from("param10"));
    param_10_matcher.param_tag = ParamEnum::Param10;
    param_10_matcher.param_indices = Some(Vec::from([9]));
    parser.add_matcher(param_10_matcher);

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
    opt_c_matcher.option_has_value = Some(OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer);
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
    opt_f_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_f_matcher);

    let mut opt_g_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionG"));
    opt_g_matcher.option_tag = OptionEnum::G;
    opt_g_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("g")]));
    opt_g_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_g_matcher);

    let mut opt_hh_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionHH"));
    opt_hh_matcher.option_tag = OptionEnum::Hh;
    opt_hh_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("HH")]));
    parser.add_matcher(opt_hh_matcher);

    let mut opt_ii_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionII"));
    opt_ii_matcher.option_tag = OptionEnum::Ii;
    opt_ii_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("ii")]));
    opt_ii_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_ii_matcher);

    let mut opt_jj_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionJJ"));
    opt_jj_matcher.option_tag = OptionEnum::Jj;
    opt_jj_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("JJ")]));
    opt_jj_matcher.option_has_value = Some(OptionHasValue::AlwaysAndValueCanStartWithOptionAnnouncer);
    parser.add_matcher(opt_jj_matcher);

    let mut opt_kkkk_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionKKKK"));
    opt_kkkk_matcher.option_tag = OptionEnum::Kkkk;
    opt_kkkk_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("kkkk")]));
    opt_kkkk_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_kkkk_matcher);

    let mut opt_ll_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionLL"));
    opt_ll_matcher.option_tag = OptionEnum::Ll;
    opt_ll_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("LL")]));
    opt_ll_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_ll_matcher);

    let mut opt_m_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionM"));
    opt_m_matcher.option_tag = OptionEnum::M;
    opt_m_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("m")]));
    opt_m_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_m_matcher);

    let mut opt_n_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionN"));
    opt_n_matcher.option_tag = OptionEnum::N;
    opt_n_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("n")]));
    opt_n_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_n_matcher);

    let mut opt_o_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionO"));
    opt_o_matcher.option_tag = OptionEnum::O;
    opt_o_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("o")]));
    opt_o_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_o_matcher);

    let mut opt_p_matcher: Matcher<OptionEnum, ParamEnum> = Matcher::new_option(String::from("optionP"));
    opt_p_matcher.option_tag = OptionEnum::P;
    opt_p_matcher.option_codes = Some(Vec::from([RegexOrText::new_text("p")]));
    opt_p_matcher.option_has_value = Some(OptionHasValue::AlwaysButValueMustNotStartWithOptionAnnouncer);
    parser.add_matcher(opt_p_matcher);

    let args = parser.parse(&command_line).unwrap();

    assert_eq!(args.len(), 27);

    for arg in args {
        match arg {
            Arg::Binary(properties) => {
                assert_eq!(properties.arg_index, 0);
                assert_eq!(properties.value_text, "binary name");
                assert_eq!(properties.line_char_index, 0);
            },
            Arg::Option(properties) => {
                match properties.matcher.option_tag {
                    OptionEnum::A => {
                        assert_eq!(properties.arg_index, 3);
                        assert_eq!(properties.option_index, 0);
                        assert_eq!(properties.code, "a");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 28);
                    },
                    OptionEnum::B => {
                        assert_eq!(properties.arg_index, 4);
                        assert_eq!(properties.option_index, 1);
                        assert_eq!(properties.code, "b");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 31);
                    },
                    OptionEnum::C => {
                        assert_eq!(properties.arg_index, 6);
                        assert_eq!(properties.option_index, 2);
                        assert_eq!(properties.code, "c");
                        assert_eq!(properties.value_text, Some(String::from("valueC_1")));
                        assert_eq!(properties.line_char_index, 41);
                    },
                    OptionEnum::D => {
                        assert_eq!(properties.arg_index, 9);
                        assert_eq!(properties.option_index, 3);
                        assert_eq!(properties.code, "d");
                        assert_eq!(properties.value_text, Some(String::from("value D1")));
                        assert_eq!(properties.line_char_index, 72);
                    },
                    OptionEnum::E => {
                        assert_eq!(properties.arg_index, 10);
                        assert_eq!(properties.option_index, 4);
                        assert_eq!(properties.code, "e");
                        assert_eq!(properties.value_text, Some(String::from("-valueE1")));
                        assert_eq!(properties.line_char_index, 86);
                    },
                    OptionEnum::F => {
                        assert_eq!(properties.arg_index, 11);
                        assert_eq!(properties.option_index, 5);
                        assert_eq!(properties.code, "F");
                        assert_eq!(properties.value_text, Some(String::from("-value F1")));
                        assert_eq!(properties.line_char_index, 98);
                    },
                    OptionEnum::G => {
                        assert_eq!(properties.arg_index, 12);
                        assert_eq!(properties.option_index, 6);
                        assert_eq!(properties.code, "g");
                        assert_eq!(properties.value_text, Some(String::from("optvalueG1")));
                        assert_eq!(properties.line_char_index, 113);
                    },
                    OptionEnum::Hh => {
                        assert_eq!(properties.arg_index, 14);
                        assert_eq!(properties.option_index, 7);
                        assert_eq!(properties.code, "hh");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.line_char_index, 134);
                    },
                    OptionEnum::Ii => {
                        assert_eq!(properties.arg_index, 15);
                        assert_eq!(properties.option_index, 8);
                        assert_eq!(properties.code, "ii");
                        assert_eq!(properties.value_text, Some(String::from("valueII1")));
                        assert_eq!(properties.line_char_index, 138);
                    },
                    OptionEnum::Jj => {
                        assert_eq!(properties.arg_index, 16);
                        assert_eq!(properties.option_index, 9);
                        assert_eq!(properties.code, "JJ");
                        assert_eq!(properties.value_text, Some(String::from("-optValueJJ1")));
                        assert_eq!(properties.line_char_index, 151);
                    },
                    OptionEnum::Kkkk => {
                        assert_eq!(properties.arg_index, 17);
                        assert_eq!(properties.option_index, 10);
                        assert_eq!(properties.code, "kkkk");
                        assert_eq!(properties.value_text, Some(String::from("valueKKKK1")));
                        assert_eq!(properties.line_char_index, 168);
                    },
                    OptionEnum::Ll => {
                        assert_eq!(properties.arg_index, 18);
                        assert_eq!(properties.option_index, 11);
                        assert_eq!(properties.code, "LL");
                        assert_eq!(properties.value_text, Some(String::from("-optValueLL1")));
                        assert_eq!(properties.line_char_index, 187);
                    },
                    OptionEnum::M => {
                        assert_eq!(properties.arg_index, 19);
                        assert_eq!(properties.option_index, 12);
                        assert_eq!(properties.code, "m");
                        assert_eq!(properties.value_text, Some(String::from("opt\"valueM")));
                        assert_eq!(properties.line_char_index, 206);
                    },
                    OptionEnum::N => {
                        assert_eq!(properties.arg_index, 20);
                        assert_eq!(properties.option_index, 13);
                        assert_eq!(properties.code, "n");
                        assert_eq!(properties.value_text, Some(String::from("opt\"valueN")));
                        assert_eq!(properties.line_char_index, 223);
                    },
                    OptionEnum::O => {
                        assert_eq!(properties.arg_index, 21);
                        assert_eq!(properties.option_index, 14);
                        assert_eq!(properties.code, "o");
                        assert_eq!(properties.value_text, Some(String::from("opt\"valueO\"")));
                        assert_eq!(properties.line_char_index, 237);
                    },
                    OptionEnum::P => {
                        assert_eq!(properties.arg_index, 22);
                        assert_eq!(properties.option_index, 15);
                        assert_eq!(properties.code, "p");
                        assert_eq!(properties.value_text, Some(String::from("opt\"\"valueP")));
                        assert_eq!(properties.line_char_index, 252);
                    },
                }
            }
            Arg::Param(properties) => {
                match properties.matcher.param_tag {
                    ParamEnum::Param1 => {
                        assert_eq!(properties.arg_index, 1);
                        assert_eq!(properties.param_index, 0);
                        assert_eq!(properties.value_text, "param1");
                        assert_eq!(properties.line_char_index, 14);
                    },
                    ParamEnum::Param2 => {
                        assert_eq!(properties.arg_index, 2);
                        assert_eq!(properties.param_index, 1);
                        assert_eq!(properties.value_text, "param2");
                        assert_eq!(properties.line_char_index, 21);
                    },
                    ParamEnum::Param3 => {
                        assert_eq!(properties.arg_index, 5);
                        assert_eq!(properties.param_index, 2);
                        assert_eq!(properties.value_text, "param3");
                        assert_eq!(properties.line_char_index, 34);
                    },
                    ParamEnum::Param4 => {
                        assert_eq!(properties.arg_index, 7);
                        assert_eq!(properties.param_index, 3);
                        assert_eq!(properties.value_text, "param4");
                        assert_eq!(properties.line_char_index, 53);
                    },
                    ParamEnum::Param5 => {
                        assert_eq!(properties.arg_index, 8);
                        assert_eq!(properties.param_index, 4);
                        assert_eq!(properties.value_text, "param 5");
                        assert_eq!(properties.line_char_index, 62);
                    },
                    ParamEnum::Param6 => {
                        assert_eq!(properties.arg_index, 13);
                        assert_eq!(properties.param_index, 5);
                        assert_eq!(properties.value_text, "param6");
                        assert_eq!(properties.line_char_index, 127);
                    },
                    ParamEnum::Param7 => {
                        assert_eq!(properties.arg_index, 23);
                        assert_eq!(properties.param_index, 6);
                        assert_eq!(properties.value_text, "par\"am7");
                        assert_eq!(properties.line_char_index, 267);
                    },
                    ParamEnum::Param8 => {
                        assert_eq!(properties.arg_index, 24);
                        assert_eq!(properties.param_index, 7);
                        assert_eq!(properties.value_text, "par\"am8");
                        assert_eq!(properties.line_char_index, 278);
                    },
                    ParamEnum::Param9 => {
                        assert_eq!(properties.arg_index, 25);
                        assert_eq!(properties.param_index, 8);
                        assert_eq!(properties.value_text, "par\"am9\"");
                        assert_eq!(properties.line_char_index, 286);
                    },
                    ParamEnum::Param10 => {
                        assert_eq!(properties.arg_index, 26);
                        assert_eq!(properties.param_index, 9);
                        assert_eq!(properties.value_text, "par\"\"am10");
                        assert_eq!(properties.line_char_index, 295);
                    },
                }
            }
        }
    }
}