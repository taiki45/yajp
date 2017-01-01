#[macro_use]
extern crate nom;
use nom::*;

use std::str;

pub fn parse(str: &'static str) -> IResult<&[u8], &[u8]> {
    named!(string, delimited!(char!('"'), is_not!("\""), char!('"')));
    named!(value, alt!(string | digit));
    named!(key_value<&[u8]>, do_parse!(k: string >> char!(':') >> opt!(multispace) >> value >> (k)));
    named!(parser, delimited!(char!('{'), key_value, char!('}')));
    return parser(str.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: to_string() to avoid lifetime invalidation.
    fn from_result(result: IResult<&[u8], &[u8]>) -> String {
        return match result {
            IResult::Done(_, o) => {
                match str::from_utf8(o) {
                    Ok(e) => e.to_string(),
                    Err(e) => panic!("Error on utf8 string conversion: {}", e),
                }
            }
            IResult::Error(e) => panic!("Parse Error: {}", e),
            IResult::Incomplete(_) => panic!("Incomplete!"),
        };
    }

    #[test]
    #[ignore]
    fn int_test() {
        let result = parse("1");
        assert_eq!(result, IResult::Done("".as_bytes(), "1".as_bytes()))
    }

    #[test]
    fn object_test() {
        let result = from_result(parse("{\"key\": \"value\"}"));
        assert_eq!(result, "key");
    }
}
