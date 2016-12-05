use num::Integer;

fn all_odd(n: u64) -> bool {
	match n.div_rem(&10) {
		(0, 1) | (0, 3) | (0, 5) | (0, 7) | (0, 9) => true,
		(div, 1) | (div, 3) | (div, 5) | (div, 7) | (div, 9) => all_odd(div),
		(_, _) => false
	}
}

fn reverse_num(n: u64, acc: u64) -> u64 {
	match n.div_rem(&10) {
		(0, rem) => acc * 10 + rem,
		(div, rem) => reverse_num(div, acc * 10 + rem)
	}
}

pub fn solve() -> u64 {
	(1u64..1_000_000_00u64)
		.filter(|&i| i % 10 != 0 && all_odd(i + reverse_num(i, 0)))
		.count() as u64
}
