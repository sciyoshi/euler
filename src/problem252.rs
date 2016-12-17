use itertools::Itertools;
use std::ops;

#[derive(Copy, Clone, Debug)]
struct Point(i64, i64);

impl ops::Sub for Point {
	type Output = Point;

	fn sub(self, other: Point) -> Point {
		Point(self.0 - other.0, self.1 - other.1)
	}
}

fn cross(a: Point, b: Point) -> i64 {
	a.0 * b.1 - a.1 * b.0
}

fn extend(points: Vec<Point>, mut candidates: Vec<Point>, area: i64) -> (Vec<Point>, i64) {
	let start = points[0];
	let last = points[points.len() - 1];
	let last2 = points[points.len() - 2];

	candidates.retain(|&p| cross(last - last2, p - last2) > 0);
	candidates.sort_by(|&p1, &p2| 0.cmp(&cross(p1 - last, p2 - last)));

	let mut possible = vec![];

	for next in candidates {
		while possible.len() > 0 && cross(next - start, possible[possible.len() - 1] - start) > 0 {
			possible.pop();
		}

		possible.push(next);
	}

	if possible.len() == 0 {
		return (points, area);
	}

	let mut max = vec![];
	let mut maxarea = 0;

	for &p in &possible {
		let mut points = points.to_vec();
		points.push(p);

		let (best, bestarea) = extend(points, possible.to_vec(), area + cross(last - start, p - start));

		if bestarea > maxarea {
			maxarea = bestarea;
			max = best;
		}
	}

	(max, maxarea)
}

pub fn solve() -> f64 {
	let points: Vec<Point> = recurrence![1, a[n]: i64 = match n {
		1 => 290797,
		_ => (a[n - 1] * a[n - 1]) % 50515093
	}].map(|k| (k % 2000) - 1000).skip(1).tuples().map(|(x, y)| Point(x, y)).take(500).collect();

	let mut maxarea = 0;

	for (i, &start) in points.iter().enumerate() {
		for &next in points[i..].iter() {
			let (best, bestarea) = extend(vec![start, next], points.to_vec(), 0);

			if bestarea > maxarea {
				maxarea = bestarea;
			}
		}
	}

	maxarea as f64 / 2.0
}