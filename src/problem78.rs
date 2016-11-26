fn pentagonal() -> impl Iterator<Item=u64> {
	(1..).scan(0, |state, x| {
		if x % 2 == 0 {
			*state += x / 2;
		} else {
			*state += x
		}

		Some(*state)
	})
}

pub fn solve() -> u64 {
	let mut partitions: Vec<i64> = vec![1];

	for n in 1.. {
		let next = pentagonal()
			.take_while(|&k| k <= n)
			.zip([true, true, false, false].iter().cycle())
			.fold(0, |sum, (k, &sign)| {
				if sign {
					sum + partitions[(n - k) as usize]
				} else {
					sum - partitions[(n - k) as usize]
				}
			}) % 1_000_000;

		partitions.push(next);

		if next == 0 {
			return n;
		}
	}

	unreachable!();
}