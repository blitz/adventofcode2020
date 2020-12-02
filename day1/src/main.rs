extern crate itertools;

use std::io::{self, BufRead};
use itertools::Itertools;

fn find_solution(values: &[i32], n: usize, expected_sum: i32) -> Option<i32>
{
    values.iter()
	.map(|v| *v)
	.combinations(n)
	.filter(|v| v.iter().sum::<i32>() == expected_sum)
	.next()
	.map(|solution| solution.iter().fold(1, |acc, x| acc * x))
}

fn main() {
    let stdin = io::stdin();

    let input_values: Vec<i32> = stdin.lock()
	.lines()
	.map(|l| l.expect("no I/O error").parse::<i32>().expect("integer input"))
	.collect();

    for n in 2..4 {
	println!("Solution for all {}-element choices : {}", n, find_solution(&input_values, n, 2020).expect("a solution"));
    }
}
