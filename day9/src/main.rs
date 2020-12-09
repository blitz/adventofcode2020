use itertools::MinMaxResult::MinMax;
use std::io::{self, BufRead};
use itertools::Itertools;

fn input_from<T: BufRead>(input: &mut T) -> Vec<i64>
{
    input
	.lines()
	.map(|l| l.expect("no I/O error"))
	.map(|l| l.parse::<i64>().expect("an integer input"))
	.collect()
}

fn is_sum_of_elem(numbers: &[i64], sum: i64) -> bool
{
    numbers.iter().tuple_combinations().find(|(&a, &b)| a + b == sum).is_some()
}

fn first_wrong_input(numbers: &[i64], prefix_size: usize) -> Option<i64>
{
    (prefix_size..numbers.len())
	.map(|p| (&numbers[(p - prefix_size)..p], numbers[p]))
	.filter(|(slice, sum)| !is_sum_of_elem(slice, *sum))
	.map(|p| p.1)
	.next()
}

fn main() {
    let numbers = input_from(&mut io::stdin().lock());

    if let Some(first_wrong) = first_wrong_input(&numbers, 25) {

	println!("Solution for 25-element prefix: {:?}",
		 first_wrong);

	let sum =
	    (2..numbers.len())
	    .filter_map(|p| numbers
			.windows(p)
			.filter(|&w| w.iter().sum::<i64>() == first_wrong)
			.next())
	    .next()
	    .map(|w| if let MinMax(a, b) = w.iter().minmax() {
		Some(a + b)
	    } else {
		None
	    })
	    .flatten();

	println!("MinMax sum: {:?}", sum);

    } else {
	println!("No solution?");
    }
}
