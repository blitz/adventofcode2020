extern crate regex;

use std::io::{self, BufRead};xo
use regex::Regex;

#[derive(Debug)]
struct Rules {
    min: usize,
    max: usize,

    pw_char: char
}

fn check_range(r: &Rules, s: &str) -> bool
{
    let count = s.chars().filter(|c| *c == r.pw_char).count();

    return count >= r.min && count <= r.max;
}

fn check_index(r: &Rules, s: &str) -> bool
{
    let chars : Vec<char> = s.chars().collect();

    return (chars[r.min - 1] == r.pw_char) ^ (chars[r.max - 1] == r.pw_char)
}

type PasswordMatcher = fn(r: &Rules, s: &str) -> bool;

fn count_matching(lines: &str, matcher: PasswordMatcher) -> usize
{
    // Recompiling the regex here is sad, but compiling it at
    // compilation time is apparently Real Hard™ in Rust. :(
    let line_re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>.): (?P<password>.*)$")
	.expect("a valid regular expression");

    lines.iter()
	.map(|l| {
	    let caps = line_re.captures(&l).expect("a matching line");
	    let rules = Rules { max : caps["max"].parse::<usize>().expect("an integer"),
				min : caps["min"].parse::<usize>().expect("an integer"),
				pw_char : caps["char"].chars().nth(0).expect("a character") };

	    matcher(&rules, &caps["password"])
	}
	)
	.filter(|b| *b)
	.count()
}


fn main() {

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock()
	.lines()
	.map(|l| l.expect("no I/O error"))
	.collect();


    let matchers: Vec<(&str, PasswordMatcher)> = vec!(("range", check_range),
						      ("index", check_index));
    
    for (name, matcher) in matchers {
	let count = count_matching(&lines, matcher);
	println!("{}: {} matching password{}.", name, count, if count != 1 { "s" } else { "" });
	
    }
}
