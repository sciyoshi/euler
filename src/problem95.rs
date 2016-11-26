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
	let limit = 50;
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

	for start in 2..limit {
		let mut num = start;
		let mut stack = vec![num];

		loop {
			let (mut next, mut status) = nums[num];

			println!("{:?} {} {:?}", stack, next, status);

			if status == Aliquot::Unknown {
				// calculate the divisor sum
				next = sieve.factor(num).unwrap().into_iter().map(|(p, e)| (p.pow((e + 1) as u32) - 1) / (p - 1)).product::<usize>() - num;
				status = Aliquot::Known;
				nums[num] = (next, status);
			}

			match nums[next] {
				(_, Aliquot::Unknown) | (_, Aliquot::Known) => {
					if let Some(dist) = stack.iter().rposition(|&e| e == next) {
						// found cycle
						status = Aliquot::Cyclic(dist as u16 + 1);
						nums[num] = (next, status);
						break;
					}

					stack.push(next);
					num = next;
					continue;
				},
				(_, Aliquot::Cyclic(k)) => { nums[num] = (next, Aliquot::Cyclic(k + 1)) },
				(_, _) => break

			}

			break;
		}
	}

	0
}
