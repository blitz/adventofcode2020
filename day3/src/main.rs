use std::io::{self, BufRead};

struct Field {
    width: usize,

    data: Vec<bool>,
}

struct Pos {
    x: usize,
    y: usize,
}

impl Field {
    fn get(&self, p: &Pos) -> Option<bool> {
	let pos = p.y * self.width + p.x % self.width;

	if pos < self.data.len() {
	    Some(self.data[pos])
	} else {
	    None
	}
    }
}

struct Step {
    x_cur: usize,
    y_cur: usize,

    x_width: usize,
    x_step: usize,
    y_step: usize,
}

impl Step {
    fn new(f: &Field, x_step: usize, y_step: usize) -> Step {
	Step { x_cur: 0,
	       y_cur: 0,

	       x_width: f.width,
	       x_step: x_step,
	       y_step: y_step,
	}
    }
}

impl Iterator for Step {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
	let p = Pos {x: self.x_cur, y: self.y_cur};

	self.x_cur = (self.x_cur + self.x_step) % self.x_width;
	self.y_cur += self.y_step;

	Some(p)
    }
}


fn to_bools(input: &str, c: char) -> Vec<bool>
{
    input.chars().map(|k| k == c).collect()
}

fn field_from<T: BufRead>(input: &mut T) -> Field {
    let lines: Vec<Vec<bool>> = input
	.lines()
	.map(|l| l.expect("no I/O error"))
	.map(|l| to_bools(&l, '#'))
	.collect();

    Field { width: lines[0].len(),
	    data: lines.into_iter().flatten().collect() }
}

fn solve(f: &Field, step: &(usize, usize)) -> usize
{
    Step::new(&f, step.0, step.1)
	.map(|p| f.get(&p))
	.take_while(|v| v.is_some())
	.filter(|v| v.expect("is some"))
	.count()
}

fn main() {
    let stdin = io::stdin();

    let field = field_from(&mut stdin.lock());

    let step_sizes : Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = step_sizes
	.iter()
	.map(|step| solve(&field, step))
	.fold(1, |acc, v| acc * v);

    println!("Product: {}", product);
}
