use itertools::Itertools;

#[derive(Clone, Debug)]
struct Node {
	rank: usize,
	parent: Option<usize>
}

struct DisjointSet {
	items: Vec<Node>,
	select_root: Box<Fn(&DisjointSet, usize, usize) -> usize>
}

impl DisjointSet {
	#[inline]
	fn find(&mut self, value: usize) -> usize {
		if let Some(parent) = self.items[value].parent {
			let parent = self.find(parent);
			self.items[value].parent = Some(parent);
			parent
		} else {
			value
		}
	}

	#[inline]
	fn union(&mut self, val1: usize, val2: usize) {
		let val1 = self.find(val1);
		let val2 = self.find(val2);

		if val1 == val2 {
			return;
		}

		let root = (*self.select_root)(&self, val1, val2);

		if root == val1 {
			self.items[val2].parent = Some(val1);
			self.items[val1].rank += self.items[val2].rank;
		} else {
			self.items[val1].parent = Some(val2);
			self.items[val2].rank += self.items[val1].rank;
		}
	}
}

pub fn solve() -> u64 {
	let lagged_fib = recurrence![55, a[n]: i64 = match n {
		1...55 => (100_003 - 200_003 * (n as i64) + 300_007 * (n as i64).pow(3)) % 1_000_000,
		_ => (a[n - 55] + a[n - 24]) % 1_000_000
	}];

	let mut friends = DisjointSet {
		items: vec![Node {rank: 1, parent: None}; 1_000_000],
		select_root: Box::new(|this, val1, val2| {
			if val1 == 524287 || val2 != 524287 && this.items[val1].rank > this.items[val2].rank {
				val1
			} else {
				val2
			}
		})
	};

	let mut count = 0;

	for (caller, callee) in lagged_fib.tuples() {
		if caller == callee {
			continue;
		}

		count += 1;
		friends.union(caller as usize, callee as usize);

		if friends.items[524287].rank >= 990_000 {
			return count;
		}
	}

	0
}