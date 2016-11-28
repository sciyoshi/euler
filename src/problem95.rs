use primal::Sieve;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Aliquot {
	Unknown,
	Known,
	Cyclic(u16),
	Escapes(u16),
	EventuallyCyclic(u16, u16)
}

fn find_cycle(sieve: &Sieve, graph: &mut Vec<(usize, Aliquot)>, el: usize, stack: &mut Vec<usize>, limit: usize) -> (Option<usize>, Aliquot) {
	if let Some(start) = stack.iter().rposition(|&n| n == el) {
		return (Some(el), Aliquot::Cyclic((stack.len() - start) as u16));
	}

	stack.push(el);

	let (mut next, mut status) = graph[el];

	if status == Aliquot::Unknown {
		// calculate the divisor sum
		next = sieve.factor(el).unwrap().into_iter().map(|(p, e)| (p.pow((e + 1) as u32) - 1) / (p - 1)).product::<usize>() - el;
		status = if next > limit { Aliquot::Escapes(0) } else { Aliquot::Known };
		graph[el] = (next, status);
	}

	if let Aliquot::Escapes(_) = status {
		stack.pop();
		return (None, status);
	}

	let (start, next_status) = find_cycle(sieve, graph, next, stack, limit);

	let (start, status) = match (start, next_status) {
		(Some(start), Aliquot::Cyclic(length)) if start == el => (None, Aliquot::Cyclic(length)),
		(Some(start), Aliquot::Cyclic(length)) => (Some(start), Aliquot::Cyclic(length)),
		(None, Aliquot::Cyclic(length)) => (None, Aliquot::EventuallyCyclic(1, length)),
		(None, Aliquot::EventuallyCyclic(dist, length)) => (None, Aliquot::EventuallyCyclic(dist + 1, length)),
		(None, Aliquot::Escapes(dist)) => (None, Aliquot::Escapes(dist + 1)),
		(_, _) => unreachable!()
	};

	graph[el] = (next, status);

	stack.pop();

	(start, status)
}

pub fn solve() -> u64 {
	let limit = 1_000_000;
	let sieve = Sieve::new(limit);

	let mut nums = vec![(1, Aliquot::Unknown); limit + 1];

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
		find_cycle(&sieve, &mut nums, start, &mut vec![], limit);
	}

	let result = (2..limit).rev().max_by_key(|&el| match nums[el] {
		(_, Aliquot::Cyclic(length)) => length,
		_ => 0
	}).unwrap();

	result as u64
}
