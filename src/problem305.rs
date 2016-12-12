use std::cmp;
use std::iter;
use itertools::Itertools;
use num::{PrimInt, ToPrimitive};

#[inline]
fn integer_length<T: PrimInt>(n: T, b: T) -> T {
	if n < b {
		T::one()
	} else {
		T::one() + integer_length(n / b, b)
	}
}

#[inline]
fn integer_length_10<T: PrimInt>(n: T) -> T {
	integer_length(n, T::from(10).unwrap())
}

fn pow_10<T: PrimInt + ToPrimitive>(n: T) -> T {
	T::from(10).unwrap().pow(n.to_u32().unwrap())
}

fn start<T: PrimInt>(n: T) -> T {
	let base = T::from(10).unwrap();

	n * integer_length_10(n) - (pow_10(integer_length_10(n)) - T::one()) / (base - T::one()) + T::one()
}

fn is_repeat(mut n: usize, d: usize, k: usize) -> bool {
	let mut val = (n / pow_10(k)) % pow_10(d);

	if k > 0 && (n % pow_10(k) != (val + 1) / pow_10(d - k)) {
		return false;
	}

	n /= pow_10(k + d);
	val -= 1;

	while n > 0 {
		let m = pow_10(cmp::min(d, integer_length_10(n)));

		if n % m != val % m || val < pow_10(d - 1) {
			return false;
		}

		n /= pow_10(d);
		val -= 1;
	}

	true
}

fn superstrings(n: usize, d: usize, k: usize) -> Box<Iterator<Item=usize>> {
	let l = integer_length_10(n);
	let s = pow_10(k) * n;
	let r = pow_10((k + l) % d);

	if l == d {
		let s = s % pow_10(d) + s / pow_10(d);

		if s < pow_10(d - 1) {
			Box::new(iter::empty())
		} else {
			Box::new(iter::once(s % pow_10(d) + s / pow_10(d)))
		}
	} else if k == 0 {
		Box::new((pow_10(d - (k + l) - 1)..pow_10(d - (k + l))).map(move |t| t * r + s))
	} else if k + l < d {
		Box::new(
			(pow_10(d - (k + l) - 1)..pow_10(d - (k + l)))
				.cartesian_product(0..pow_10(k))
				.map(move |(t, b)| t * r + s + b))
	} else if k + l == d {
		Box::new((0..pow_10(k)).map(move |t| t + s))
	} else {
		let s = s % pow_10(d) + s / pow_10(d);

		if s < pow_10(d - 1) {
			Box::new(iter::empty())
		} else {
			Box::new((0..pow_10(d - l)).map(move |t| t * r + s))
		}
	}
}

fn count(n: usize, d: usize) -> usize {
	if d < integer_length_10(n) {
		(0..d).map(|k| is_repeat(n, d, k) as usize).sum::<usize>()
	} else {
		(0..d).map(|k| superstrings(n, d, k)).kmerge().dedup().count()
	}
}

fn start_kth(n: usize, mut k: usize) -> usize {
	for d in 1.. {
		let c = count(n, d);

		if k > c {
			k -= c;
			continue;
		} else {
			let result = (0..d).map(|k| superstrings(n, d, k).map(move |v| (v, k))).kmerge().dedup().nth(k - 1).unwrap();

			return start(result.0) + d - (integer_length_10(n) + result.1 - 1) % d - 1;
		}
	}

	unreachable!()
}

pub fn solve() -> u64 {
	(1..14).map(|n| 3.pow(n)).map(|n| start_kth(n, n)).sum::<usize>() as u64
}
