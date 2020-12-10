use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::HashMap;

fn choices(memo: &mut HashMap<(i32, Vec<i32>), usize>, cur: i32, v: &[i32]) -> usize
{
    if let Some(v) = memo.get(&(cur, v.into())) {
	*v
    } else {
	let r =
	    if v.is_empty() {
		1
	    } else {
		assert!(cur <= v[0]);

		let diff_to_first = v[0] - cur;

		if diff_to_first > 3 {
		    0
		} else {
		    if v.len() > 1 {
			let f = choices(memo, cur, &v[1..]);

			f + choices(memo, v[0], &v[1..])
		    } else {
			1
		    }
		}
	    };

	memo.insert((cur, v.into()), r);
	r
    }
}

fn main() {
    let sorted_input: Vec<i32> = io::stdin().lock()
	.lines()
	.map(|l| l.expect("no I/O error").parse::<i32>().expect("integer input"))
	.sorted()
	.collect();

    let sorted_with_terminals: Vec<i32> = sorted_input.iter()
	.merge(&[0, 3 + *sorted_input.last().expect("at least one element")])
	.cloned()
	.collect();


    let differences: Vec<i32> = sorted_with_terminals.iter()
	.skip(1)
	.zip(sorted_with_terminals.iter())
	.map(|(a, b)| a - b)
	.collect();

    let ones = differences.iter().filter(|&&x| x == 1).count();
    let threes = differences.iter().filter(|&&x| x == 3).count();

    println!("{} {} -> {}", ones, threes, ones * threes);

    let mut memo = HashMap::new();
    println!("choices {}", choices(&mut memo, 0, &sorted_with_terminals[1..sorted_with_terminals.len()-1]));
}
