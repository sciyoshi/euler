use primal::Sieve;

fn m(p: u64, q: u64, n: u64) -> u64 {
	if p * q > n {
		return 0;
	}

	let pmax = (n as f64 / q as f64).log(p as f64) as u64;

	let mut exp = (pmax, 1);
	let mut val = p.pow(pmax as u32) * q;
	let mut max = 0;

	while exp.0 >= 1 {
		if val <= n && val > max {
			max = val;
		}

		if val <= n {
			val *= q;
			exp.1 += 1;
		} else {
			val /= p;
			exp.0 -= 1;
		}
	}

	max
}

pub fn solve() -> u64 {
	let limit = 10_000_000;
	let sieve = Sieve::new(limit);
	let root = (limit as f64).sqrt() as usize;
	let mut total = 0;

	for p in sieve.primes_from(2).take_while(|&p| p <= root) {
		for q in sieve.primes_from(p + 1).take_while(|&q| p * q <= limit) {
			total += m(p as u64, q as u64, limit as u64);
		}
	}

	total
}
