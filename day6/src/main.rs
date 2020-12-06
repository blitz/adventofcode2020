use std::io::{stdin, Result};
use helpers::{to_lines};
use std::collections::BTreeSet;

type AnswerSet = BTreeSet<u32>;

fn to_answer(s: &str) -> AnswerSet {
    s.chars().map(|s| s as u32 - 'a' as u32).collect()
}

fn anyone_answered(v: &Vec<AnswerSet>) -> usize {
    let empty_set = AnswerSet::new();

    v.iter().fold(empty_set, |acc, a| acc.union(&a).cloned().collect()).len()
}

fn everyone_answered(v: &Vec<AnswerSet>) -> usize {
    let empty_set: AnswerSet = (0..26).collect();

    v.iter().fold(empty_set, |acc, a| acc.intersection(&a).cloned().collect()).len()
}


fn main() -> Result<()> {
    let lines: Vec<String> = to_lines(&mut stdin().lock())?
	.iter()
	.map(|s| s.trim().into())
	.collect::<Vec<String>>();

    let groups: Vec<Vec<AnswerSet>> = lines
	.split(|s| s.is_empty())
	.map(|v| v.iter().map(|s| to_answer(s)).collect())
	.collect();

    println!("Sum of answered questions (any): {}",
	     groups.iter().map(anyone_answered).sum::<usize>());

    println!("Sum of answered questions (all): {}",
	     groups.iter().map(everyone_answered).sum::<usize>());

    Ok(())
}
