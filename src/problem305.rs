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

fn is_repeat(n: usize, d: usize, k: usize) -> bool {
	let l = integer_length_10(n);
	let r = (l + d - 1) / d;
	let g = (pow_10(d * (r + 1)) - 1) / (pow_10(d) - 1);
	let f = (pow_10(d * (r + 2)) - (pow_10(d) - 1) * (r + 2) - 1) / ((pow_10(d) - 1) * (pow_10(d) - 1));

	let shifted = (n / pow_10(k)) % pow_10(d);

	integer_length_10(shifted) == d && shifted >= r && (((shifted - r) * g + f) / pow_10(d - k)) % pow_10(l) == n
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

fn kth(n: usize, mut k: usize) -> usize {
	for d in 1.. {
		let c = count(n, d);

		println!("{}: {}", d, c);

		if k > c {
			k -= c;
			continue;
		} else {
			let result = (0..d).map(|k| superstrings(n, d, k).map(move |v| (v, k))).kmerge().dedup().nth(k - 1).unwrap();

			return start(result.0) + d - result.1;
		}
	}

	unreachable!()
}

pub fn solve() -> u64 {
	println!("{}", kth(5, 5));
	println!("{}", kth(7780, 7780));

	for n in 1..14 {
		let k = 3.pow(n);

		println!("===> {} = {}", k, kth(k, k));
	}

	0
}

/*
37
169
2208
4725
161013
926669
14199388
 */
