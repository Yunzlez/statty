use std::io::{Error, ErrorKind, Result};
use chrono::Datelike;

use diesel::data_types::PgInterval;
use diesel::dsl::{IntervalDsl, now};
use diesel::internal::derives::as_expression::Bound;
use diesel::internal::derives::numeric_ops::Sub;
use diesel::sql_types::Interval;
use lazy_static::lazy_static;
use nom::bytes::complete::take_while_m_n;
use nom::combinator::map_res;
use nom::IResult;
use nom::character::complete::one_of;
use regex::Regex;

lazy_static! {
    static ref PERIOD_REGEX: Regex = Regex::new(r"^(?:\d{1,2}[dmy]|all)$").unwrap();
}

//parse a period string, like 7d, 1m, 1y, all into a diesel DSL interval
pub fn parse_period(period: String) -> Result<Sub<now, Bound<Interval, PgInterval>>> {
    if !validate_period_str(&period) {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid format"));
    }
    let current_date = chrono::offset::Local::now().naive_local();

    if period == "all" {
        return Ok(now - (current_date.date().year() - 1970).years()) // 1970 is the epoch, so that will return all
    }

    let (num, unit) = do_parse(period).unwrap();

    let period = match unit.as_str() {
        "d" => Ok(now - num.days()),
        "m" => Ok(now - num.months()),
        "y" => Ok(now - num.years()),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid period specified"))
    };

    return period
}

fn validate_period_str(period: &String) -> bool {
    return PERIOD_REGEX.is_match(period);
}

fn do_parse(period: String) -> Result<(i32, String)> {
    let (input, num) = parse_digits(period.as_str()).unwrap();
    let (_, unit) = parse_unit(input).unwrap();

    return Ok((num, unit));
}

fn parse_digits(input: &str) -> IResult<&str, i32> {
    map_res(
        take_while_m_n(1,2,|c: char| c.is_digit(10)),
        |s: &str|Ok::<i32, std::num::ParseIntError>(s.parse::<i32>().unwrap())
    )(input)
}

fn parse_unit(input: &str) -> IResult<&str, String> {
    map_res(
        one_of("dmy"),
        |u| Ok::<String, Error>(u.to_string())
    )(input)
}
