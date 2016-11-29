use primal::Sieve;

pub fn solve() -> u64 {
	let sieve = Sieve::new(10_000_000_000);

	let mut sums = vec![vec![0u64; 10]; 10];

	for p in sieve.primes_from(1_000_000_000) {
		let mut counts = vec![0; 10];
		let mut digits = p;

		while digits != 0 {
			counts[digits % 10] += 1;
			digits /= 10;
		}

		for (i, &count) in counts.iter().enumerate() {
			sums[i][count] += p as u64;
		}
	}

	sums.iter().map(|l| l.iter().take_while(|&s| *s > 0).last().unwrap()).sum::<u64>()
}