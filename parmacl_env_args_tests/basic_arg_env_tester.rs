use parmacl::{Parser, Arg};
fn main() {
    // const COMMAND_LINE: &str = "binary param1 param2 -a -b param3";
    let parser: Parser = Parser::with_env_args_defaults();
    let args = parser.parse_env().unwrap();

    assert_eq!(args.len(), 6);

    let mut first_non_binary_arg_env_line_approximate_char_index: usize = 0;

    for arg in args {
        match arg {
            Arg::Binary(properties) => {
                assert_eq!(properties.arg_index, 0);
                assert_eq!(properties.env_arg_index, 0);
                assert!(properties.value_text.contains("basic_arg_env_tester"));
                assert_eq!(properties.char_index, 0);
                assert_eq!(properties.env_line_approximate_char_index, 0);
            },
            Arg::Option(properties) => {
                match properties.option_index {
                    0 => {
                        assert_eq!(properties.arg_index, 3);
                        assert_eq!(properties.env_arg_index, 3);
                        assert_eq!(properties.code, "a");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.char_index, 0);
                        assert_eq!(properties.env_line_approximate_char_index, first_non_binary_arg_env_line_approximate_char_index + 14);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 4);
                        assert_eq!(properties.env_arg_index, 4);
                        assert_eq!(properties.code, "b");
                        assert_eq!(properties.value_text, None);
                        assert_eq!(properties.char_index, 0);
                        assert_eq!(properties.env_line_approximate_char_index, first_non_binary_arg_env_line_approximate_char_index + 17);
                    },
                    _ => {
                        panic!("Unexpected option");
                    },
                }
            }
            Arg::Param(properties) => {
                match properties.param_index {
                    0 => {
                        first_non_binary_arg_env_line_approximate_char_index = properties.env_line_approximate_char_index;

                        assert_eq!(properties.arg_index, 1);
                        assert_eq!(properties.env_arg_index, 1);
                        assert_eq!(properties.value_text, "param1");
                        assert_eq!(properties.char_index, 0);
                        assert_eq!(properties.env_line_approximate_char_index, first_non_binary_arg_env_line_approximate_char_index);
                    },
                    1 => {
                        assert_eq!(properties.arg_index, 2);
                        assert_eq!(properties.env_arg_index, 2);
                        assert_eq!(properties.value_text, "param2");
                        assert_eq!(properties.char_index, 0);
                        assert_eq!(properties.env_line_approximate_char_index, first_non_binary_arg_env_line_approximate_char_index + 7);
                    },
                    2 => {
                        assert_eq!(properties.arg_index, 5);
                        assert_eq!(properties.env_arg_index, 5);
                        assert_eq!(properties.value_text, "param3");
                        assert_eq!(properties.char_index, 0);
                        assert_eq!(properties.env_line_approximate_char_index, first_non_binary_arg_env_line_approximate_char_index + 20);
                    },
                    _ => {
                        panic!("Unexpected param");
                    },
                }
            }
        }
    }
}
