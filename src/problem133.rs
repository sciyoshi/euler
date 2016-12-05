use mod_pow::ModPow;
use primal::Sieve;
use num::{pow, one, BigInt, BigUint, FromPrimitive};

pub fn solve() -> u64 {
	let sieve = Sieve::new(100_000);
	let mut total: u64 = 2 + 3;

	'primes: for prime in sieve.primes_from(5) {
		for n in 1..30 {
			let exp = pow(BigInt::from_u64(10).unwrap(), n);

			if BigInt::from_u64(10).unwrap().mod_pow(&exp, &BigUint::from_u64(prime as u64).unwrap()) == one() {
				continue 'primes;
			}
		}

		total += prime as u64;
	}

	total
}