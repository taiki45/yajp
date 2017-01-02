#[macro_use]
extern crate nom;
use nom::*;

use std::str;
use std::str::FromStr;
use std::collections::HashMap;

pub mod json {
    // TODO: Extend number for float value.
    #[derive(PartialEq, Debug)]
    pub enum Value {
        Number(i64),
        String(::std::string::String),
        Object(::HashMap<::std::string::String, Value>),
    }
}

pub fn parse(str: &'static str) -> IResult<&[u8], json::Value> {
    return value(str.as_bytes());
}

named!(value<json::Value>, alt!(
    string => {|s| json::Value::String(String::from(s)) } |
    integer => {|i| json::Value::Number(i) } |
    object => {|h| json::Value::Object(h) }
));

named!(object<HashMap<::std::string::String, json::Value>>, map!(
    delimited!(char!('{'), key_values, char!('}')),
    |kvs| {
        let mut h = HashMap::new();
        for (k, v) in kvs {
            h.insert(String::from(k), v);
        }
        h
    }
));

named!(key_values<Vec<(&str, json::Value)>>, separated_list!(char!(','), key_value));

named!(key_value<(&str, json::Value)>, do_parse!(
    opt!(multispace) >>
    k: string >>
    opt!(multispace) >>
    char!(':') >>
    opt!(multispace) >>
    v: value >>
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

    fn obj(k: &'static str, v: json::Value) -> json::Value {
        let mut h = HashMap::new();
        h.insert(String::from(k), v);
        return json::Value::Object(h);
    }

    #[test]
    fn object_test() {
        let result = extact_output(parse("{\"key\":\"value\"}"));
        let v = json::Value::String(String::from("value"));
        assert_eq!(result, obj("key", v));
    }

    #[test]
    fn object_with_number_test() {
        let result = extact_output(parse("{\"key\":1}"));
        let v = json::Value::Number(1);
        assert_eq!(result, obj("key", v));
    }

    #[test]
    fn object_with_spaces_test() {
        let result = extact_output(parse("{\n\"key\": \n\"value\"\n}"));
        let v = json::Value::String(String::from("value"));
        assert_eq!(result, obj("key", v));
    }
}
