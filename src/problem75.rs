use std::collections::VecDeque;
use roaring::RoaringBitmap;

#[derive(Copy, Clone, Debug)]
struct Triple(u64, u64, u64);

impl Triple {
	fn sum(&self) -> u64 {
		self.0 + self.1 + self.2
	}

	fn children(&self) -> [Self; 3] {
		let &Triple(a, b, c) = self;

		[
			Triple(a + 2 * c - 2 * b, 2 * a + 2 * c - b, 2 * a + 3 * c - 2 * b),
			Triple(a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c),
			Triple(2 * b + 2 * c - a, b + 2 * c - 2 * a, 2 * b + 3 * c - 2 * a)
		]
	}
}

pub fn solve() -> u64 {
	let limit = 1_500_000;
	let mut single: RoaringBitmap<u64> = RoaringBitmap::new();
	let mut multiple: RoaringBitmap<u64> = RoaringBitmap::new();

	let mut queue: VecDeque<Triple> = VecDeque::new();

	queue.push_back(Triple(3, 4, 5));

	while let Some(triple) = queue.pop_front() {
		let sum = triple.sum();

		if sum > limit {
			continue;
		}

		queue.extend(&triple.children());

		for total in (sum...limit).step_by(sum) {
			if !single.insert(total) {
				multiple.insert(total);
			}
		}
	}

	(single - multiple).len()
}
