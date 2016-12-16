use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use itertools::Itertools;
use utils::linkedmatrix::LinkedMatrix;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Guess {
	Column(u8, u8),
	Row(u8, u8),
	Square(u8, u8, u8),
	Cell(u8, u8),
	Digit(u8, u8, u8),
	None
}

impl ToString for Guess {
	fn to_string(&self) -> String {
		match *self {
			Guess::Column(a, b) => format!("C{}={}", a, b),
			Guess::Row(a, b) => format!("R{}={}", a, b),
			Guess::Square(a, b, c) => format!("S{}:{}={}", a, b, c),
			Guess::Cell(a, b) => format!("{}:{}", a, b),
			Guess::Digit(a, b, c) => format!("{}:{}={}", a, b, c),
			Guess::None => "".to_string()
		}
	}
}

impl Default for Guess {
	fn default() -> Self {
		Guess::None
	}
}

type Sudoku = [[u8; 9]; 9];

fn solve_sudoku(s: &mut Sudoku) {
	let mut m = LinkedMatrix::new();

	for i in 0..9 {
		for digit in 1...9 {
			m.add_column(Guess::Column(i, digit));
			m.add_column(Guess::Row(i, digit));
			m.add_column(Guess::Square(i / 3, i % 3, digit));
		}

		for j in 0..9 {
			m.add_column(Guess::Cell(i, j));
		}
	}

	for (i, &row) in s.iter().enumerate() {
		let i = i as u8;
		for (j, &val) in row.iter().enumerate() {
			let j = j as u8;
			if val == 0 {
				for digit in 1...9 {
					m.add_constraint(Guess::Digit(i, j, digit), &vec![
						Guess::Column(j, digit),
						Guess::Row(i, digit),
						Guess::Square(j / 3, i / 3, digit),
						Guess::Cell(j, i)
					]);
				}
			} else {
				m.add_constraint(Guess::Digit(i, j, val), &vec![
					Guess::Column(j, val),
					Guess::Row(i, val),
					Guess::Square(j / 3, i / 3, val),
					Guess::Cell(j, i)
				]);
			}
		}
	}

	let solution = m.solve().unwrap();

	for digit in solution {
		if let Guess::Digit(i, j, digit) = digit {
			s[i as usize][j as usize] = digit;
		}
	}
}

pub fn solve() -> u64 {
	let f = BufReader::new(File::open("data/p096_sudoku.txt").unwrap());
	let mut total = 0;

	for chunk in &f.lines().chunks(10) {
		let mut s: Sudoku = [[0u8; 9]; 9];

		for (i, line) in chunk.skip(1).enumerate() {
			for (j, c) in line.unwrap().chars().enumerate() {
				s[i][j] = c.to_string().parse().unwrap();
			}
		}

		solve_sudoku(&mut s);

		total += s[0][0] as u64 * 100 + s[0][1] as u64 * 10 + s[0][2] as u64;
	}

	total
}