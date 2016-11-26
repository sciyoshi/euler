fn is_square(val: u64) -> bool {
	let sqrt = (val as f64).sqrt() as u64;

	sqrt * sqrt == val
}

pub fn solve() -> u64 {
	let limit = 1_000_000_000 / 3;
	let mut sum = 0;

	for i in (3...limit).step_by(2) {
		if is_square((3 * i + 1) * (i - 1)) {
			sum += 3 * i + 1;
		}
	}

	for i in (3...limit).step_by(2) {
		if is_square((3 * i - 1) * (i + 1)) {
			sum += 3 * i - 1;
		}
	}

	sum
}