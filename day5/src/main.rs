use std::collections::BTreeSet;
use std::io::{self, BufRead};

type SeatId = i32;
type SeatIdSet = BTreeSet<SeatId>;

fn to_seat_id(s: &str) -> SeatId
{
    s
	.chars()
	.map(|c| match c {
	    'F' | 'L' => 0,
	    'B' | 'R' => 1,
	    _ => panic!("Invalid seat character: {}", c)
	})
	.fold(0, |acc, c| (acc << 1) | c)
}

fn input_from<T: BufRead>(input: &mut T) -> Vec<String>
{
    input
	.lines()
	.map(|l| l.expect("no I/O error"))
	.collect()
}

fn main() {
    let seats : SeatIdSet = input_from(&mut io::stdin().lock())
	.iter()
	.map(|s| to_seat_id(s))
	.collect();

    let highest_seat = seats
	.iter().rev().cloned().next()	// There is .last() as unstable API.
	.expect("at least one element");

    println!("Highest seat ID: {}", highest_seat);

    let missing_seat =
	(0..highest_seat)
	.find(|v| !seats.contains(v) && seats.contains(&(v - 1)) && seats.contains(&(v + 1)))
	.expect("one missing seat");

    println!("Missing seat ID: {}", missing_seat);
}
