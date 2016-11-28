use num::PrimInt;

fn count(len: u32) -> i64 {
	let t = len - 1;

	2 * (16.pow(t) - 2 * 15.pow(t) + 14.pow(t)) + 13 * (16.pow(t) - 3 * 15.pow(t) + 3 * 14.pow(t) - 13.pow(t))
}

pub fn solve() -> String {
	format!("{:X}", (1...16).map(|n| count(n)).sum::<i64>())
}