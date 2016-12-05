use ndarray::Array;
use num::Num;
use std::cmp::Ord;
use std::collections::VecDeque;

struct LaggedFib {
	terms: VecDeque<i64>
}

impl Iterator for LaggedFib {
	type Item = i64;

	fn next(&mut self) -> Option<i64> {
		let next = (self.terms[0] + self.terms[31] + 1_000_000) % 1_000_000 - 500_000;

		self.terms.push_back(next);

		self.terms.pop_front()
	}
}

fn lagged_fib() -> LaggedFib {
	LaggedFib {
		terms: (1i64...55).map(|k| {
			(100_003 - 200_003 * k + 300_007 * k.pow(3)) % 1_000_000 - 500_000
		}).collect()
	}
}

fn max_sum<T: IntoIterator>(items: T) -> T::Item where T::Item: Num + Ord {
	items.into_iter().scan(T::Item::zero(), |state, el| {
		Some(state)
	}).max()
}

pub fn solve() -> i64 {
	let arr = Array::from_shape_vec((10, 10), lagged_fib().take(100).collect()).unwrap();

	println!("{:?}", arr);

	let result = max_sum(arr.slice(s![0..1, ..]));

	println!("{:?}", result);

	0
}