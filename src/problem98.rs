use utils::numbers::{integer_length_10, integer_digits_10};
use std::collections::HashSet;
use std::cmp;

const PAIRS: &'static [(&'static str, &'static str)] = &[
	("NO", "ON"),
	("ACT", "CAT"),
	("DOG", "GOD"),
	("EAT", "TEA"),
	("HOW", "WHO"),
	("ITS", "SIT"),
	("NOW", "OWN"),
	("CARE", "RACE"),
	("DEAL", "LEAD"),
	("EARN", "NEAR"),
	("EAST", "SEAT"),
	("FILE", "LIFE"),
	("FORM", "FROM"),
	("HATE", "HEAT"),
	("ITEM", "TIME"),
	("MALE", "MEAL"),
	("MEAN", "NAME"),
	("NOTE", "TONE"),
	("POST", "SPOT"),
	("POST", "STOP"),
	("RATE", "TEAR"),
	("SHUT", "THUS"),
	("SIGN", "SING"),
	("SPOT", "STOP"),
	("SURE", "USER"),
	("ARISE", "RAISE"),
	("BOARD", "BROAD"),
	("EARTH", "HEART"),
	("LEAST", "STEAL"),
	("NIGHT", "THING"),
	("PHASE", "SHAPE"),
	("QUIET", "QUITE"),
	("SHEET", "THESE"),
	("SHOUT", "SOUTH"),
	("THROW", "WORTH"),
	("CENTRE", "RECENT"),
	("COURSE", "SOURCE"),
	("CREDIT", "DIRECT"),
	("DANGER", "GARDEN"),
	("EXCEPT", "EXPECT"),
	("FORMER", "REFORM"),
	("IGNORE", "REGION"),
	("CREATION", "REACTION"),
	("INTRODUCE", "REDUCTION")
];

pub fn solve() -> u64 {
	let limit = 999999999f64.sqrt().ceil() as u64;
	let squares: Vec<u64> = (1...limit).map(|n| n * n).collect();
	let squaresmap: HashSet<u64> = squares.iter().cloned().collect();
	let mut max = 0;

	for s in squares {
		'words: for &(w1, w2) in PAIRS {
			if integer_length_10(s) == w1.len() as u64 {
				let digits = integer_digits_10(s);
				let mut map: [Option<char>; 10] = [None; 10];

				for (&d, c) in digits.iter().zip(w1.chars()) {
					if let Some(c2) = map[d as usize] {
						if c != c2 {
							continue 'words;
						}
					}

					map[d as usize] = Some(c);
				}

				let mut val2 = 0;

				for (i, &v) in map.iter().enumerate() {
					if let Some(v) = v {
						for (j, c) in w2.chars().rev().enumerate() {
							if c == v {
								val2 += 10u64.pow(j as u32) * i as u64;
							}
						}
					}
				}

				if integer_length_10(val2) == w2.len() as u64 && squaresmap.contains(&val2) {
					max = cmp::max(max, cmp::max(s, val2));
				}
			}
		}
	}

	max
}
