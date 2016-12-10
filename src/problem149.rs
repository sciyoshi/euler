use ndarray::{Array, Axis};
use num::Num;
use std::ops::Deref;
use std::cmp::{Ord, max};
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

fn max_sum<T: IntoIterator, V: Num + Ord + Copy>(items: T) -> V where T::Item: Deref<Target=V> {
	items.into_iter().scan(V::zero(), |state, el| {
		*state = max(V::zero(), *state + *el);

		Some(*state)
	}).max().unwrap_or(V::zero())
}

pub fn solve() -> i64 {
	let dim = 2_000;
	let arr = Array::from_shape_vec((dim, dim), lagged_fib().take(dim * dim).collect()).unwrap();

	let row_max = arr.axis_iter(Axis(0)).map(max_sum).max().unwrap();
	let col_max = arr.axis_iter(Axis(1)).map(max_sum).max().unwrap();

	let mut diag1_max = 0;
	let mut diag2_max = 0;

	for i in 0..10 {
		diag1_max = max(diag1_max, max_sum((0...i).zip((0...i).rev()).map(|(i, j)| &arr[[i, j]])));
		diag2_max = max(diag2_max, max_sum((dim - 1 - i...dim - 1).zip(0...i).map(|(i, j)| &arr[[i, j]])));
	}

	for i in 1..10 {
		diag1_max = max(diag1_max, max_sum((i...dim - 1).zip((i...dim - 1).rev()).map(|(i, j)| &arr[[i, j]])));
		diag2_max = max(diag2_max, max_sum((0...i).zip(i...dim - 1).map(|(i, j)| &arr[[i, j]])));
	}

	max(max(row_max, col_max), max(diag1_max, diag2_max))
}