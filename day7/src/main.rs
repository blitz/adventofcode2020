use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

type Color = String;

// Holds a bag rule as described in the puzzle input. This is only
// used for parsing and is then converted to `BagRuleMap`.
#[derive(Debug)]
struct BagRule {
    color: Color,
    contains: Vec<(usize, Color)>
}

// Our internal representation of the bag rules.
type ColorSet = BTreeSet<Color>;
type BagRuleMap = BTreeMap<Color, ColorSet>;

fn to_rule_map(rules: &Vec<BagRule>) -> BagRuleMap
{
    rules.iter().map(|r| (r.color.clone(), r.contains.iter().cloned().map(|r| r.1).collect())).collect()
}

// Returns the set of colors that are reachable via the rules from a
// starting set of colors.
//
// The resulting set of colors is always a superset of the given set
// of colors.
fn reachable_from(rules: &BagRuleMap, colors: &ColorSet) -> ColorSet
{
    colors
	.iter()
	.map(|c| rules[c].clone())
	.fold(colors.clone(), |acc, cs| &acc | &cs)
}

// Transitively expands the bag rules for one step.
//
// So if A can contain B and B can contain C, a new rule will be added
// for A to contain C.
fn transitive_expand(rules: &BagRuleMap) -> BagRuleMap {
    rules.iter().map(|(color, contains)| (color.clone(), reachable_from(rules, contains))).collect()
}

// Repeatedly apply `transitive_expand` until a fixpoint is
// reached. This is then the transitive closure of the bag rules.
fn transitive_closure(rules: &BagRuleMap) -> BagRuleMap {
    let expanded = transitive_expand(rules);

    if *rules == expanded {
	rules.clone()
    } else {
	transitive_closure(&expanded)
    }
}

// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.

fn to_bag_rule(s: &str) -> Option<BagRule>
{
    let simple_re = Regex::new(r"(.+) bags contain no other bags.").expect("a valid regex");
    let complex_re = Regex::new(r"(.+) bags contain (.*).").expect("a valid regex");
    let bag_re = Regex::new(r",?\s*(\d+) (.+?) bags?").expect("a valid regex");


    if let Some(m) = simple_re.captures(s) {
	Some(BagRule { color: m[1].into(), contains: [].into() })
    } else if let Some(m) = complex_re.captures(s) {
	let contents: Vec<(usize, Color)> = bag_re.captures_iter(&m[2].to_string())
	    .map(|c| (c[1].to_string().parse::<usize>().expect("digits"), c[2].to_string()))
	    .collect();

	Some(BagRule { color: m[1].into(), contains: contents })

    } else {
	None
    }
}

fn input_from<T: BufRead>(input: &mut T) -> Vec<String>
{
    input
	.lines()
	.map(|l| l.expect("no I/O error"))
	.collect()
}

fn count_bags(rules: &Vec<BagRule>, color: &Color) -> usize
{
    if let Some(m) = rules.iter().find(|r| *r.color == *color) {
	m.contains.iter().fold(0, |acc, (n, c)| acc + n * (1 + count_bags(rules, c)))
    } else {
	0
    }
}


fn main() {
    let parsed_rules : Vec<BagRule> = input_from(&mut io::stdin().lock())
	.iter()
	.map(|s| to_bag_rule(s).expect("a valid rule"))
	.collect();

    let rule_map = to_rule_map(&parsed_rules);
    let closure = transitive_closure(&rule_map);

    println!("colors that reach shiny gold: {}",
	     closure.values().filter(|s| s.contains("shiny gold")).count());

    println!("Shiny gold bag contains {} bags.",
	     count_bags(&parsed_rules, &"shiny gold".to_string()));
}
