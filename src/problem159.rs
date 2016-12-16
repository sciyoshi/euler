use primal::Sieve;
use std::cmp;

fn mdrs(factors: &Vec<(usize, usize)>) -> u64 {
	let mut powers = [0usize; 10];

	for &(prime, power) in factors {
		powers[prime % 9] += power;
	}

	let nines = powers[3] / 2;
	powers[9] += nines;
	powers[3] -= nines * 2;

	let eights = cmp::min(powers[4], powers[2]);
	powers[8] += eights;
	powers[4] -= eights;
	powers[2] -= eights;

	let eights = powers[2] / 3;
	powers[8] += eights;
	powers[2] -= eights * 3;

	let sixes = cmp::min(powers[3], powers[2]);
	powers[6] += sixes;
	powers[3] -= sixes;
	powers[2] -= sixes;

	powers.iter().enumerate().map(|(d, p)| d * p).sum::<usize>() as u64
}

pub fn solve() -> u64 {
	let limit = 1_000_000;
	let sieve = Sieve::new(limit);

	(2..limit).map(|n| mdrs(&sieve.factor(n).unwrap())).sum()
}