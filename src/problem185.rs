use utils::linkedmatrix::LinkedMatrix;
use time;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Guess {
	Digit(usize, usize),
	Constraint(usize),
	Hint(u64),
	None
}

impl ToString for Guess {
	fn to_string(&self) -> String {
		match *self {
			Guess::Digit(a, b) => format!("{}:{}", a, b),
			Guess::Constraint(a) => format!("[{}]", a),
			Guess::Hint(a) => format!("{:.6}", a.to_string()),
			Guess::None => "".to_string()
		}
	}
}

impl Default for Guess {
	fn default() -> Self {
		Guess::None
	}
}

pub fn solve() -> u64 {
	let mut m = LinkedMatrix::new();

	let size = 16;

	let guesses = vec![
		(Guess::Hint(5616185650518293), 2),
		(Guess::Hint(3847439647293047), 1),
		(Guess::Hint(5855462940810587), 3),
		(Guess::Hint(9742855507068353), 3),
		(Guess::Hint(4296849643607543), 3),
		(Guess::Hint(3174248439465858), 1),
		(Guess::Hint(4513559094146117), 2),
		(Guess::Hint(7890971548908067), 3),
		(Guess::Hint(8157356344118483), 1),
		(Guess::Hint(2615250744386899), 2),
		(Guess::Hint(8690095851526254), 3),
		(Guess::Hint(6375711915077050), 1),
		(Guess::Hint(6913859173121360), 1),
		(Guess::Hint(6442889055042768), 2),
		(Guess::Hint(2321386104303845), 0),
		(Guess::Hint(2326509471271448), 2),
		(Guess::Hint(5251583379644322), 2),
		(Guess::Hint(1748270476758276), 3),
		(Guess::Hint(4895722652190306), 1),
		(Guess::Hint(3041631117224635), 3),
		(Guess::Hint(1841236454324589), 3),
		(Guess::Hint(2659862637316867), 2)
	];

	// let size = 5;

	// let guesses = vec![
	// 	(Guess::Hint(90342), 2),
	// 	(Guess::Hint(70794), 0),
	// 	(Guess::Hint(39458), 2),
	// 	(Guess::Hint(34109), 1),
	// 	(Guess::Hint(51545), 2),
	// 	(Guess::Hint(12531), 1)
	// ];

	for &(guess, multiplicity) in &guesses {
		m.add_column_with_multiplicity(guess, multiplicity);
	}

	for digit in 1...size {
		m.add_column(Guess::Constraint(digit));
	}

	let mut constraints = vec![];

	for pos in 1...size {
		for digit in 0...9 {
			let mut row = vec![Guess::Constraint(pos)];

			for &(guess, _) in &guesses {
				if let Guess::Hint(val) = guess {
					if (val / 10u64.pow((size - pos) as u32)) % 10 == digit as u64 {
						row.push(Guess::Hint(val));
					}
				}
			}

			constraints.push((Guess::Digit(pos, digit), row));
		}
	}

	constraints.sort_by_key(|&(_, ref row)| row.len());
	constraints.reverse();

	for (guess, row) in constraints {
		m.add_constraint(guess, &row);
	}

	let start = time::PreciseTime::now();

	let solution = m.solve().unwrap();

	println!("duration: {}", start.to(time::PreciseTime::now()));

	let mut result = 0;

	for guess in solution {
		if let Guess::Digit(pos, digit) = guess {
			result += digit as u64 * 10u64.pow((size - pos) as u32);
		}
	}

	result
}
