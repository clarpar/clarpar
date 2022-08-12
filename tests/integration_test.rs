use clarpar::{Parser, Matcher, Arg};

#[test]
fn first() {
    enum OptionEnum {
        // A,
        // B,
    }
    // enum ParamEnum {
    //     X,
    //     Y,
    // }

    let mut parser: Parser<OptionEnum> = Parser::new();
    let mut matcher: Matcher<OptionEnum> = Matcher::new(String::from("x"));
    matcher.param_tag = Some(1);
    parser.add_matcher(matcher);
    let args = parser.parse("-o").expect("Test fail");
    println!("Arg count: {}", args.len());
    for arg in args {
        match arg {
            Arg::Option(properties) => {
                println!("Option code: {}", properties.code);
            }
            Arg::Param(properties) => {
                println!("Param value: {}", properties.value_text);
            }
        }
    }
}