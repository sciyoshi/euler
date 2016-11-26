use primal::Sieve;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Aliquot {
	Unknown,
	Known,
	Cyclic(u16),
	Escapes(u16),
	EventuallyCyclic(u16, u16)
}

pub fn solve() -> u64 {
	let limit = 1_000;
	let sieve = Sieve::new(limit);

	let mut nums = vec![(1, Aliquot::Unknown); limit];

	nums[1] = (1, Aliquot::Cyclic(1));

	// precalculate divisor function for prime powers

	for prime in sieve.primes_from(2) {
		let mut sum = 1;
		let mut power = prime;

		while power < limit {
			nums[power] = (sum, Aliquot::Known);
			sum += power;
			power *= prime;
		}
	}

	let mut stack = vec![];

	for num in 2..limit {
		stack.push(num);

		while !stack.is_empty() {
			let (mut next, mut status) = nums[*stack.last().unwrap()];

			if status == Aliquot::Unknown {
				// calculate the divisor sum
				next = sieve.factor(num).unwrap().into_iter().map(|(p, e)| (p.pow((e + 1) as u32) - 1) / (p - 1)).product::<usize>() - num;
				status = Aliquot::Known;
			}

			nums[num] = (next, status);

			println!("{}, {}, {:?}", num, next, status);

			stack.pop();
		}
	}

	0
}
