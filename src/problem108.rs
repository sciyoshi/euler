use std::iter::Iterator;
use primal::Sieve;

struct Factors<'a> {
	factors: &'a Vec<(usize, usize)>,
	powers: Vec<usize>
}

impl<'a> Iterator for Factors<'a> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let result = self.factors.iter().zip(self.powers.iter()).map(|(&(prime, _), &power)| prime.pow(power as u32)).product();
		let mut done = true;

		for (i, power) in self.powers.iter_mut().enumerate() {
			if *power == self.factors[i].1 {
				*power = 0
			} else {
				*power += 1;
				done = false;
				break;
			}
		}

		if done { None } else { Some(result) }
	}
}

fn divisors(factors: &Vec<(usize, usize)>) -> Factors {
	Factors {
		factors: factors,
		powers: vec![0; factors.len()]
	}
}

fn reciprocals(sieve: &Sieve, val: usize) -> u64 {
	let factors = sieve.factor(val).unwrap();

	1 + divisors(&factors).map(|k| 2u64.pow(sieve.factor(val / k).unwrap().len() as u32) / 2).sum::<u64>()
}

pub fn solve() -> u64 {
	let sieve = Sieve::new(1_000_000);

	for num in 5.. {
		if reciprocals(&sieve, num) > 1_000 {
			return num as u64;
		}
	}

	unreachable!();
}