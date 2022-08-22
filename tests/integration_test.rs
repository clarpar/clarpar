use parmacl::{Parser, Arg, RegexOrText, OptionHasValue};
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
    let args = parser.parse_line(COMMAND_LINE).unwrap();

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
    let command_line = String::from(BASIC_MATCHERS_COMMAND_LINE);

    let mut parser: Parser<OptionEnum, ParamEnum> = Parser::new();

    parser
        .push_new_param_matcher("param1")
            .set_param_tag(ParamEnum::Param1)
            .some_param_indices(&[0]);

    parser
        .push_new_param_matcher("param2")
            .set_param_tag(ParamEnum::Param2)
            .some_param_indices(&[1]);

    parser
        .push_new_param_matcher("param3")
            .set_param_tag(ParamEnum::Param3)
            .some_param_indices(&[2]);

    parser
        .push_new_param_matcher("param4")
            .set_param_tag(ParamEnum::Param4)
            .some_param_indices(&[3]);

    parser
        .push_new_param_matcher("param5")
            .set_param_tag(ParamEnum::Param5)
            .some_param_indices(&[4]);

    parser
        .push_new_param_matcher("param6")
            .set_param_tag(ParamEnum::Param6)
            .some_param_indices(&[5]);

    parser
        .push_new_param_matcher("param7")
            .set_param_tag(ParamEnum::Param7)
            .some_param_indices(&[6]);

    parser
        .push_new_param_matcher("param8")
            .set_param_tag(ParamEnum::Param8)
            .some_param_indices(&[7]);

    parser
        .push_new_param_matcher("param9")
            .set_param_tag(ParamEnum::Param9)
            .some_param_indices(&[8]);

    parser
        .push_new_param_matcher("param10")
            .set_param_tag(ParamEnum::Param10)
            .some_param_indices(&[9]);

    parser
        .push_new_option_matcher("optionA")
            .set_option_tag(OptionEnum::A)
            .some_option_codes(&[RegexOrText::new_text("a")]);

    parser
        .push_new_option_matcher("optionB")
            .set_option_tag(OptionEnum::B)
            .some_option_codes(&[RegexOrText::new_text("b")]);

    parser
        .push_new_option_matcher("optionC")
            .set_option_tag(OptionEnum::C)
            .some_option_codes(&[RegexOrText::new_text("c")])
            .some_option_has_value(OptionHasValue::Always)
            .set_option_value_can_start_with_option_announcer(true);

    parser
        .push_new_option_matcher("optionD")
            .set_option_tag(OptionEnum::D)
            .some_option_codes(&[RegexOrText::new_text("d")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionE")
            .set_option_tag(OptionEnum::E)
            .some_option_codes(&[RegexOrText::new_text("E")])
            .some_option_has_value(OptionHasValue::Always)
            .set_option_value_can_start_with_option_announcer(true);

    parser
        .push_new_option_matcher("optionF")
            .set_option_tag(OptionEnum::F)
            .some_option_codes(&[RegexOrText::new_text("f")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionG")
            .set_option_tag(OptionEnum::G)
            .some_option_codes(&[RegexOrText::new_text("g")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionHH")
            .set_option_tag(OptionEnum::Hh)
            .some_option_codes(&[RegexOrText::new_text("HH")]);

    parser
        .push_new_option_matcher("optionII")
            .set_option_tag(OptionEnum::Ii)
            .some_option_codes(&[RegexOrText::new_text("ii")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionJJ")
            .set_option_tag(OptionEnum::Jj)
            .some_option_codes(&[RegexOrText::new_text("JJ")])
            .some_option_has_value(OptionHasValue::Always)
            .set_option_value_can_start_with_option_announcer(true);

    parser
        .push_new_option_matcher("optionKKKK")
            .set_option_tag(OptionEnum::Kkkk)
            .some_option_codes(&[RegexOrText::new_text("kkkk")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionLL")
            .set_option_tag(OptionEnum::Ll)
            .some_option_codes(&[RegexOrText::new_text("LL")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionM")
            .set_option_tag(OptionEnum::M)
            .some_option_codes(&[RegexOrText::new_text("m")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionN")
            .set_option_tag(OptionEnum::N)
            .some_option_codes(&[RegexOrText::new_text("n")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionO")
            .set_option_tag(OptionEnum::O)
            .some_option_codes(&[RegexOrText::new_text("o")])
            .some_option_has_value(OptionHasValue::Always);

    parser
        .push_new_option_matcher("optionP")
            .set_option_tag(OptionEnum::P)
            .some_option_codes(&[RegexOrText::new_text("p")])
            .some_option_has_value(OptionHasValue::Always);

    let args = parser.parse_line(&command_line).unwrap();

    assert_eq!(args.len(), 27);

    for arg in args {
        match arg {
            Arg::Binary(properties) => {
                assert_eq!(properties.arg_index, 0);
                assert_eq!(properties.value_text, "binary name");
                assert_eq!(properties.line_char_index, 0);
            },
            Arg::Option(properties) => {
                match properties.matcher.option_tag() {
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
                match properties.matcher.param_tag() {
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
