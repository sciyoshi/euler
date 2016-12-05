use primal::Sieve;
use std::iter::repeat;
use std::cmp;

struct Products {
	limit: u64,
	exps: Vec<u32>
}

impl Iterator for Products {
	type Item = Vec<u32>;

	fn next(&mut self) -> Option<Vec<u32>> {
		let ref mut exps = self.exps;

		let result = exps.clone();

		if exps.is_empty() {
			return None;
		}

		while let Some(&1) = exps.last() {
			exps.pop();
		}

		if exps.is_empty() {
			return Some(result);
		}

		*exps.last_mut().unwrap() -= 1;

		let product = (3u64..).step_by(2).zip(exps.iter()).map(|(n, &k)| n.pow(k)).product::<u64>();
		let next = 1 + 2 * (exps.len() as u64 + 1);

		exps.push((self.limit as f64 / product as f64).log(next as f64).ceil() as u32);

		Some(result)
	}
}

fn products(limit: u64) -> Products {
	Products {
		limit: limit,
		exps: vec![(limit as f64).log(3f64).ceil() as u32]
	}
}

pub fn solve() -> u64 {
	let sieve = Sieve::new(1_000_000);
	let limit = 4_000_000 * 2 - 1;
	let mut min: Option<u64> = None;

	for powers in products(limit) {
		let expanded: Vec<usize> = powers.iter().enumerate().rev().flat_map(|(p, &r)| repeat(p + 1).take(r as usize)).collect();

		let val: Option<u64> = sieve
			.primes_from(2)
			.zip(expanded)
			.map(|(p, e)| (p as u64).pow(e as u32))
			.fold(Some(1), |acc, v| acc.and_then(|a| a.checked_mul(v)));

		if let Some(v) = val {
			min = min.map(|m| cmp::min(m, v)).or(Some(v));
		}
	}

	min.unwrap() as u64
}