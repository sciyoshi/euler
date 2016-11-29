use primal::Sieve;

pub fn solve() -> u64 {
	let limit = 100_000;
	let sieve = Sieve::new(limit);

	// simple brute-force. could be smarter and sort by radical, but this is easier

	let mut result: Vec<(usize, usize)> = (1...limit).map(|n| (sieve.factor(n).unwrap().iter().map(|&(p, _)| p).product(), n)).collect();

	result.sort();

	result[10_000 - 1].1 as u64
}