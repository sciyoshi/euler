use primal::Sieve;
use num::{FromPrimitive, ToPrimitive, BigInt, BigUint, Integer};
use mod_pow::ModPow;

pub fn solve() -> u64 {
	let sieve = Sieve::new(1_000_000_000);

	for (p, n) in sieve.primes_from(3).zip(2..) {
		let p1: BigInt = FromPrimitive::from_usize(p + 1).unwrap();
		let p2: BigInt = FromPrimitive::from_usize(p - 1).unwrap();
		let pi: BigInt = FromPrimitive::from_usize(p).unwrap();
		let p: BigUint = FromPrimitive::from_usize(p).unwrap();
		let n: BigInt = FromPrimitive::from_usize(n).unwrap();
		let ps = &p * &p;
		let psi = &pi * &pi;

		if (p1.mod_pow(&n, &ps) + p2.mod_pow(&n, &ps)).mod_floor(&psi) > FromPrimitive::from_u64(10_000_000_000).unwrap() {
			return n.to_u64().unwrap()
		}
	}

	unreachable!();
}

