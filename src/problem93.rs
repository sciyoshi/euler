use std::rc::Rc;
use num::traits::{Num, NumCast};
use roaring::RoaringBitmap;

#[derive(Copy, Clone, Debug)]
enum Op {
	Add,
	Sub,
	RSub,
	Mul,
	Div,
	RDiv
}

#[derive(Clone, Debug)]
enum Node<V: Num + Copy + 'static> {
	Op(Op, Rc<Node<V>>, Rc<Node<V>>),
	Val(V)
}

impl<V: Num + Copy + 'static> Node<V> {
	fn val(value: V) -> Rc<Node<V>> {
		Rc::new(Node::Val(value))
	}

	fn eval(&self) -> V {
		match *self {
			Node::Val(v) => v,
			Node::Op(Op::Add, ref left, ref right) => left.eval() + right.eval(),
			Node::Op(Op::Sub, ref left, ref right) => left.eval() - right.eval(),
			Node::Op(Op::RSub, ref left, ref right) => right.eval() - left.eval(),
			Node::Op(Op::Mul, ref left, ref right) => left.eval() * right.eval(),
			Node::Op(Op::Div, ref left, ref right) => left.eval() / right.eval(),
			Node::Op(Op::RDiv, ref left, ref right) => right.eval() / left.eval()
		}
	}
}

fn add_leaf<V: Num + Copy + 'static>(tree: Rc<Node<V>>, val: V) -> Box<Iterator<Item=Node<V>>> {
	let ops = vec![Op::Add, Op::Sub, Op::RSub, Op::Mul, Op::Div, Op::RDiv];

	let cloned = tree.clone();

	let iter = ops.clone().into_iter().map(move |op| {
		Node::Op(op, Node::val(val), cloned.clone())
	});

	if let Node::Op(op, ref left, ref right) = *tree {
		let (l, r) = (left.clone(), right.clone());

		let result = iter
			.chain(add_leaf(left.clone(), val).into_iter()
				.map(move |el| Node::Op(op, Rc::new(el), r.clone())))
			.chain(add_leaf(right.clone(), val).into_iter()
				.map(move |el| Node::Op(op, l.clone(), Rc::new(el))));

		Box::new(result)
	} else {
		Box::new(iter)
	}
}

fn expr_trees<V: Num + Copy + 'static>(terms: &[V]) -> Box<Iterator<Item=Node<V>>> {
	let (&first, rest) = terms.split_first().unwrap();

	if rest.is_empty() {
		Box::new(vec![Node::Val(first)].into_iter())
	} else {
		Box::new(expr_trees(rest)
			.flat_map(move |tree| add_leaf(Rc::new(tree), first)))
	}
}

#[inline]
fn k_subset_lex_next<V: Num + NumCast + Copy>(mut set: Vec<V>, k: usize, n: V) -> Option<Vec<V>> {
	let mut i = k;

	while i > 0 && set[i - 1] == n - V::from(k).unwrap() + V::from(i).unwrap() {
		i -= 1;
	}

	if i == 0 {
		None
	} else {
		let last = set[i - 1];
		for j in i...k {
			set[j - 1] = last + V::one() + V::from(j).unwrap() - V::from(i).unwrap();
		}
		Some(set)
	}
}

pub fn solve() -> u64 {
	let mut set: Option<Vec<f64>> = Some(vec![1., 2., 3., 4.]);
	let mut max = 0;
	let mut best = vec![];

	while let Some(cur) = set {
		let mut reachable = RoaringBitmap::new();

		for tree in expr_trees::<f64>(&cur) {
			let val = tree.eval();

			if val.is_normal() && val >= 1. && val == val.floor() {
				reachable.insert(val as u32);
			}
		}

		let expressible = (1..).zip(reachable.iter()).take_while(|&(i, v)| i == v).count();

		if expressible > max {
			max = expressible;
			best = cur.clone();
		}

		set = k_subset_lex_next(cur, 4, 9.);
	}

	best.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap()
}
