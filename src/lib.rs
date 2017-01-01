#[macro_use]
extern crate nom;
use nom::*;

use std::str;

pub fn parse(str: &'static str) -> IResult<&[u8], (&str, &str)> {
    named!(string<&str>, map_res!(delimited!(char!('"'), is_not!("\""), char!('"')), str::from_utf8));
    named!(key_value<(&str, &str)>, do_parse!(
            opt!(multispace) >>
            k: string >>
            opt!(multispace) >>
            char!(':') >>
            opt!(multispace) >>
            v: string >>
            opt!(multispace) >>
            (k, v))
          );
    named!(parser<(&str, &str)>, delimited!(char!('{'), key_value, char!('}')));
    return parser(str.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extact_output<I>(result: IResult<&[u8], I>) -> I {
        return match result {
            IResult::Done(_, o) => o,
            IResult::Error(e) => panic!("Parse Error: {}", e),
            IResult::Incomplete(_) => panic!("Incomplete!"),
        };
    }

    #[test]
    fn object_test() {
        let result = extact_output(parse("{\"key\":\"value\"}"));
        assert_eq!(result, ("key", "value"));
    }

    #[test]
    fn object_with_spaces_test() {
        let result = extact_output(parse("{\n\"key\": \n\"value\"\n}"));
        assert_eq!(result, ("key", "value"));
    }
}
