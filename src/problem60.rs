use std::collections::{HashMap, HashSet};
use primal;

fn next_power_of_10(val: usize) -> usize {
	match val {
		0...10 => 10,
		10...100 => 100,
		100...1000 => 1000,
		1000...10000 => 10000,
		10000...100000 => 100000,
		100000...1000000 => 1000000,
		1000000...10000000 => 10000000,
		10000000...100000000 => 100000000,
		100000000...1000000000 => 1000000000,
		1000000000...10000000000 => 10000000000,
		10000000000...100000000000 => 100000000000,
		100000000000...1000000000000 => 1000000000000,
		_ => 1
	}
}

fn extend_clique<'a>(graph: &'a HashMap<usize, HashSet<usize>>, stack: &mut Vec<usize>, candidates_stack: &mut Vec<&'a HashSet<usize>>, depth: usize) -> Option<usize> {
	if stack.len() >= depth {
		return Some(stack.iter().sum::<usize>());
	}

	let mut min = None;

	for &next in candidates_stack.last().unwrap().iter() {
		if !candidates_stack.iter().all(|cand| cand.contains(&next)) {
			continue;
		}

		stack.push(next);
		candidates_stack.push(graph.get(&next).unwrap());

		if let Some(val) = extend_clique(graph, stack, candidates_stack, depth) {
			if min.map_or(true, |cur| cur > val) {
				min = Some(val);
			}
		}

		stack.pop();
		candidates_stack.pop();
	}

	min
}

pub fn solve() -> u64 {
	let limit = 10_000_000_000;
	let sieve = primal::Sieve::new(limit);

	let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

	for prime1 in sieve.primes_from(3).take_while(|&x| x < 10_000) {
		let size1 = next_power_of_10(prime1);

		for prime2 in sieve.primes_from(prime1 + 1).take_while(|&x| x < 10_000) {
			let size2 = next_power_of_10(prime2);

			if sieve.is_prime(prime1 * size2 + prime2) && sieve.is_prime(prime2 * size1 + prime1) {
				graph.entry(prime1).or_insert_with(|| HashSet::new()).insert(prime2);
				graph.entry(prime2).or_insert_with(|| HashSet::new()).insert(prime1);
			}
		}
	}

	let mut min = None;

	for start in sieve.primes_from(3).take_while(|&x| x < 10_000) {
		if let Some(candidates) = graph.get(&start) {
			let mut stack = vec![start];
			let mut candidates_stack = vec![candidates];

			if let Some(val) = extend_clique(&graph, &mut stack, &mut candidates_stack, 5) {
				if min.map_or(true, |cur| cur > val) {
					min = Some(val);
				}
			}
		}
	}

	min.unwrap() as u64
}

