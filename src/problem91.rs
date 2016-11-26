use std::cmp;
use num::Rational;

macro_rules! frac {
	($x: tt / $y: tt) => (Rational::new($x, $y));
}

pub fn solve() -> u64 {
	let bound = 50;

	// triangles with right angles at origin
	let mut total = bound * bound;

	// triangles with right angles on x/y axes
	total += bound * bound * 2;

	for x in 1...bound {
		for y in 1...x {
			let slope = frac!(y / x);
			let (&yp, &xp) = (slope.numer(), slope.denom());

			let min = cmp::max(-frac!(y / xp), frac!((x - bound) / yp));
			let max = cmp::min(frac!(x / yp), frac!((bound - y) / xp));

			let triangles = max.floor().to_integer() - min.ceil().to_integer();

			if y == x {
				total += triangles;
			} else {
				// triangles with right angles not on diagonal can be reflected
				total += triangles * 2;
			}
		}
	}

	total as u64
}
