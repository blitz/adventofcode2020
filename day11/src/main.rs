use std::convert::TryInto;
use std::io::{self, BufRead};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
	match c {
	    '.' => Tile::Floor,
	    'L' => Tile::Empty,
	    '#' => Tile::Occupied,
	    _ => panic!("Unrecognized field: {}", c),
	}
    }
}

type Pos = (isize, isize);

struct TileIter {
    max: Pos,
    cur: Pos,
}

impl Iterator for TileIter {
    type Item = Pos;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
	if self.cur.0 < self.max.0 && self.cur.1 < self.max.1 {
	    let old = self.cur.clone();

	    self.cur.1 += 1;

	    if self.cur.1 == self.max.1 {
		self.cur.0 += 1;
		self.cur.1 = 0;
	    }

	    Some(old)
	} else {
	    None
	}
    }
}

type Tiles = Vec<Vec<Tile>>;

fn index_iter(v: &Tiles) -> TileIter {
    TileIter {
	max: (v.len().try_into().unwrap(), v[0].len().try_into().unwrap()),
	cur: (0, 0)
    }
}

fn in_bounds(tiles: &Tiles, pos: Pos) -> bool
{
    pos.0 >= 0 && pos.1 >= 0
	&& pos.0 < tiles.len().try_into().unwrap()
	&& pos.1 < tiles[0].len().try_into().unwrap()
}

fn get(tiles: &Tiles, pos: Pos) -> Tile
{
    if in_bounds(tiles, pos) {
	let a: usize = pos.0.try_into().unwrap();
	let b: usize = pos.1.try_into().unwrap();

	tiles[a][b]
    } else {
	Tile::Floor
    }
}

fn adjacent(tiles: &Tiles, pos: Pos) -> [Tile; 8]
{
    [get(tiles, (pos.0 - 1, pos.1 - 1)),
     get(tiles, (pos.0 - 1, pos.1 + 0)),
     get(tiles, (pos.0 - 1, pos.1 + 1)),

     get(tiles, (pos.0 + 0, pos.1 - 1)),
     get(tiles, (pos.0 + 0, pos.1 + 1)),

     get(tiles, (pos.0 + 1, pos.1 - 1)),
     get(tiles, (pos.0 + 1, pos.1 + 0)),
     get(tiles, (pos.0 + 1, pos.1 + 1)),
    ]
}

fn tile_step(tiles: &Tiles, pos: Pos) -> Tile
{
    let adj = adjacent(tiles, pos);

    match get(tiles, pos) {
	Tile::Floor => Tile::Floor,
	Tile::Empty => if adj.iter().all(|&v| v != Tile::Occupied) {
	    Tile::Occupied
	} else {
	    Tile::Empty
	},
	Tile::Occupied => if adj.iter().filter(|&&v| v == Tile::Occupied).count() >= 4 {
	    Tile::Empty
	} else {
	    Tile::Occupied
	}
    }
}

fn step(tiles: &Tiles) -> Tiles
{
    index_iter(tiles).map(|p| tile_step(tiles, p))
	.collect::<Vec<Tile>>()
	.chunks(tiles[0].len())
	.map(|c| c.to_vec())
	.collect()
}

fn main() {
    let input: Vec<Vec<Tile>> = io::stdin().lock()
	.lines()
	.map(|l| l.expect("no I/O error").chars().map(|c| c.into()).collect())
	.collect();

    let mut state = input.clone();

    loop {
	let new_state = step(&state);

	if new_state == state {
	    break;
	}

	state = new_state
    }

    println!("Occupied: {}", state.iter().flatten().filter(|&&v| v == Tile::Occupied).count());
}
