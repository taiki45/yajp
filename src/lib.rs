#[macro_use]
extern crate nom;
use nom::*;

use std::str;
use std::str::FromStr;

pub mod json {
    // TODO: Extend number for float value.
    #[derive(PartialEq, Debug)]
    pub enum Value {
        Number(i64),
        String(::std::string::String),
    }
}

pub fn parse(str: &'static str) -> IResult<&[u8], (&str, json::Value)> {
    return object(str.as_bytes());
}

named!(object<(&str, json::Value)>, delimited!(char!('{'), key_value, char!('}')));

named!(key_value<(&str, json::Value)>, do_parse!(
    opt!(multispace) >>
    k: string >>
    opt!(multispace) >>
    char!(':') >>
    opt!(multispace) >>
    v: alt!(
        string => {|s| json::Value::String(String::from(s)) } |
        integer => {|i| json::Value::Number(i) }
    ) >>
    opt!(multispace) >>
    (k, v))
);

named!(string<&str>, map_res!(delimited!(char!('"'), is_not!("\""), char!('"')), str::from_utf8));

named!(integer<i64>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

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
        assert_eq!(result, ("key", json::Value::String(String::from("value"))));
    }

    #[test]
    fn object_with_number_test() {
        let result = extact_output(parse("{\"key\":1}"));
        assert_eq!(result, ("key", json::Value::Number(1)));
    }

    // #[test]
    // fn object_with_spaces_test() {
    // let result = extact_output(parse("{\n\"key\": \n\"value\"\n}"));
    // assert_eq!(result, ("key", "value"));
    // }
}
