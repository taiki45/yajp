#[macro_use]
extern crate nom;
use nom::*;

use std::str;

pub fn parse(str: &'static str) -> nom::IResult<&[u8], &[u8]> {
    named!(obj, delimited!(char!('{'), is_not!("}"), char!('}')));
    named!(parser, alt!(digit | obj));
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
                    Err(e) => panic!("{}", e),
                }
            }
            IResult::Error(e) => panic!("{}", e),
            IResult::Incomplete(_) => panic!("Incomplete!"),
        };
    }

    #[test]
    fn int_test() {
        let result = parse("1");
        assert_eq!(result, IResult::Done("".as_bytes(), "1".as_bytes()))
    }

    #[test]
    fn object_test() {
        let result = from_result(parse("{\"a\": 1}"));
        assert_eq!(result, "\"a\": 1");
    }
}
