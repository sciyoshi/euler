fn digit_sum(n: u64) -> u64 {
	if n < 10 {
		n
	} else {
		n % 10 + digit_sum(n / 10)
	}
}

pub fn solve() -> u64 {
	let mut solutions = vec![];

	for root in 2..100 {
		let mut power = root * root;

		while power < 1_000_000_000_000_000 {
			if root == digit_sum(power) {
				solutions.push(power);

				if solutions.len() == 30 {
					solutions.sort();
					break;
				}
			}

			power *= root;
		}
	}

	*solutions.last().unwrap()
}