use std::mem;

fn trailing_zeros(mut n: u64) -> u8 {
	if n == 0 {
		mem::size_of::<u64>() as u8
	} else {
		let mut count = 0;

		n = (n ^ (n - 1)) >> 1;

		while n != 0 {
			n >>= 1;
			count += 1;
		}

		count
	}
}

pub fn solve() -> u64 {
	let mut d1 = 0b10011110100101u64;

	while d1 != 0 {
		let zeros = trailing_zeros(d1);

		println!("{}", zeros);

		d1 >>= zeros + 1;
	}



	0
}