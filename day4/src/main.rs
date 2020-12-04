use std::io::{self, BufRead};
use std::collections::BTreeMap;
use std::iter::Iterator;
use itertools::Itertools;

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

fn main() -> Result<(), ParseError> {
    let records: Vec<KeyValueMap> = string_records_from(&mut io::stdin().lock())
	.iter()
	.map(|r| to_key_value_list(&r))
	.collect::<Result<Vec<KeyValueMap>, ParseError>>()?;

    let (valid, invalid): (Vec<KeyValueMap>, Vec<KeyValueMap>) = records
	.iter().cloned()
	.partition(|r| is_valid_record(r));

    for v in &valid {
	println!("  valid: {:?}", v);
    }

    for iv in &invalid {
	println!("invalid: {:?}", iv);
    }

    println!("valid={} invalid={}", valid.len(), invalid.len());
    Ok(())
}
