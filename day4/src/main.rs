#[macro_use] extern crate lazy_static;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{self, BufRead};
use std::iter::Iterator;

type Key = String;
type Value = String;

type KeyValue = (Key, Value);

type KeyValueMap = BTreeMap<Key, Value>;

#[derive(Debug)]
enum ParseError { ParseError }

fn to_key_value(s: &str) -> Result<KeyValue, ParseError> {
    let pair: Vec<&str> = s.splitn(2, ':').collect();

    if pair.len() == 2 {
	Ok((pair[0].into(), pair[1].into()))
    } else {
	Err(ParseError::ParseError)
    }
}

fn to_key_value_list(s: &str) -> Result<KeyValueMap, ParseError> {
    s.split_whitespace().map(to_key_value).collect()
}

fn string_records_from<T: BufRead>(input: &mut T) -> Vec<String> {
    let trimmed_lines: Vec<String> = input
	.lines()
	.map(|s| s.expect("no I/O error"))
	.map(|s| s.trim().into())
	.collect();

    // With every line trimmed, we can now split them into subgroups
    // and join individual subgroups into single strings.
    trimmed_lines
	.split(|s| s.is_empty())
	.map(|g| g.join(" "))
	.collect()
}

fn sorted<T: Ord+Clone>(v: &Vec<T>) -> Vec<T> {
    v.into_iter().sorted().cloned().collect()
}


fn is_valid_record(kv_map: &KeyValueMap) -> bool {
    let keys: Vec<String> = kv_map.keys().cloned().collect();

    let necessary_keys = sorted(&vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]);
    let necessary_keys2 = sorted(&vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    keys == necessary_keys || keys == necessary_keys2
}

fn integer_between_incl(s: &str, lo: isize, hi: isize) -> bool {
    match s.parse::<isize>() {
	Ok(v) => v >= lo && v <= hi,
	Err(_) => false
    }
}

fn length_exactly(s: &str, length: usize) -> bool {
    s.len() == length
}

fn integer_between_incl_unit(s: &str, unit: &str, lo: isize, hi: isize) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(\D+)$").expect("a valid regex");
    }

    match RE.captures(s) {
	Some(c) => integer_between_incl(c.get(1).unwrap().as_str(), lo, hi) && c.get(2).unwrap().as_str() == unit,
	None => false
    }
}

fn is_color_value(s: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").expect("a valid regex");
    }

    RE.is_match(s)
}

fn one_of(s: &str, cand: &[&str]) -> bool
{
    cand.iter().find(|&&x| x == s).is_some()
}

fn all_digits(s: &str) -> bool
{
    s.chars().all(|s| s.is_digit(10))
}

fn is_valid_field(key: &Key, val: &Value) -> bool {
    match key.as_str() {
	"byr" => integer_between_incl(val, 1920, 2002) && length_exactly(val, 4),
	"iyr" => integer_between_incl(val, 2010, 2020) && length_exactly(val, 4),
	"eyr" => integer_between_incl(val, 2020, 2030) && length_exactly(val, 4),
	"hgt" => integer_between_incl_unit(val, "cm", 150, 193) || integer_between_incl_unit(val, "in", 59, 76),
	"hcl" => is_color_value(val),
	"ecl" => one_of(val, &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]),
	"pid" => all_digits(val) && length_exactly(val, 9),
	"cid" => true,
	_ => todo!()
    }
}

fn has_valid_keys(kv_map: &KeyValueMap) -> bool {
    kv_map.iter().all(|kv| is_valid_field(kv.0, kv.1))
}

fn main() -> Result<(), ParseError> {
    let records: Vec<KeyValueMap> = string_records_from(&mut io::stdin().lock())
	.iter()
	.map(|r| to_key_value_list(&r))
	.collect::<Result<Vec<KeyValueMap>, ParseError>>()?;

    let (valid, invalid): (Vec<KeyValueMap>, Vec<KeyValueMap>) = records
	.iter().cloned()
	.partition(|r| is_valid_record(r));

    println!("valid={} invalid={}", valid.len(), invalid.len());
    println!("valid_keys={}", valid.iter().filter(|m| has_valid_keys(m)).count());

    Ok(())
}
