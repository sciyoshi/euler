use bit_set::BitSet;

struct Ulam {
	terms: Vec<u64>,
	init: [u64; 2],
	found_even: bool
}

impl Ulam {
	fn new(a: u64, b: u64) -> Self {
		Ulam {
			terms: vec![],
			init: [a, b],
			found_even: false
		}
	}

	fn has_unique_sum(&self, val: u64) -> bool {
		let mut front = self.terms.iter().peekable();
		let mut back = self.terms.iter().rev().peekable();
		let mut found = false;

		while front.peek().unwrap() < back.peek().unwrap() {
			let total = *front.peek().unwrap() + *back.peek().unwrap();

			if total == val && found {
				return false;
			} else if total == val && !found {
				found = true;
				front.next();
				back.next();
			} else if total > val {
				back.next();
			} else if total < val {
				front.next();
			}
		}

		found
	}
}

impl Iterator for Ulam {
	type Item = u64;

	fn next(&mut self) -> Option<u64> {
		let val = match self.terms.len() {
			0 => self.init[0],
			1 => self.init[1],
			_ => (self.terms.last().unwrap() + 1..).filter(|&x| self.has_unique_sum(x)).next().unwrap()
		};

		self.terms.push(val);

		if val > 2 && val % 2 == 0 {
			self.found_even = true;
		}

		Some(val)
	}
}

struct Ulam2 {
	a: BitSet,
	b: BitSet,
	n: usize,
	m: usize
}

impl Ulam2 {
	fn new(a: usize, b: usize) -> Self {
		Ulam2 {
			a: vec![a as usize, b as usize].iter().cloned().collect(),
			b: BitSet::with_capacity(2 * b),
			n: 1,
			m: a
		}
	}
}

impl Iterator for Ulam2 {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		let result = self.m;

		self.n += 1;
		self.m += 1;

		while !self.a.contains(self.m) && self.m < self.a.capacity() {
			self.m += 1;
		}

		self.a.reserve_len(self.m * 2);
		self.b.reserve_len(self.m * 2);

		Some(result)
	}
}

pub fn solve() -> u64 {
	println!("{:?}", Ulam::new(1, 2).take(10).collect::<Vec<u64>>());
	println!("{:?}", Ulam2::new(1, 2).take(10).collect::<Vec<usize>>());

	0
}