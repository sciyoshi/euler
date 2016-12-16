use prettytable;
use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::hash::Hash;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

trait LinkedNode: Sized {
	type Iter: Iterator<Item=Self>;

	fn link(self) -> Self;
	fn remove_vertical(&self);
	fn remove_horizontal(&self);
	fn insert_vertical(&self);
	fn insert_horizontal(&self);
	fn append_down(&self, other: &Self);
	fn append_right(&self, other: &Self);
	fn append_up(&self, other: &Self);
	fn append_left(&self, other: &Self);
	fn iter(&self, direction: Direction) -> Self::Iter;
}

#[derive(Clone, Debug)]
struct Node<T> {
	val: T,
	multiplicity: Cell<usize>,
	size: Cell<usize>,
	left: RefCell<Weak<Node<T>>>,
	right: RefCell<Weak<Node<T>>>,
	up: RefCell<Weak<Node<T>>>,
	down: RefCell<Weak<Node<T>>>,
	column: RefCell<Weak<Node<T>>>
}

impl<T> Node<T> {
	fn new(val: T) -> Self {
		Node {
			val: val,
			multiplicity: Cell::new(1),
			size: Cell::new(0),
			left: RefCell::new(Weak::new()),
			right: RefCell::new(Weak::new()),
			up: RefCell::new(Weak::new()),
			down: RefCell::new(Weak::new()),
			column: RefCell::new(Weak::new())
		}
	}

	fn new_for_column(val: T, column: &Rc<Node<T>>) -> Self {
		Node {
			val: val,
			multiplicity: Cell::new(1),
			size: Cell::new(0),
			left: RefCell::new(Weak::new()),
			right: RefCell::new(Weak::new()),
			up: RefCell::new(Weak::new()),
			down: RefCell::new(Weak::new()),
			column: RefCell::new(Rc::downgrade(column))
		}
	}
}

struct NodeIterator<T> {
	start: Rc<Node<T>>,
	cur: Option<Rc<Node<T>>>,
	dir: Direction
}

impl<T> Iterator for NodeIterator<T> {
	type Item = Rc<Node<T>>;

	fn next(&mut self) -> Option<Self::Item> {
		let result = match self.cur {
			None => return None,
			Some(ref node) => node.clone()
		};

		let next = match self.dir {
			Direction::Up => result.up.borrow().upgrade(),
			Direction::Down => result.down.borrow().upgrade(),
			Direction::Left => result.left.borrow().upgrade(),
			Direction::Right => result.right.borrow().upgrade(),
		};

		if let Some(node) = next {
			if Rc::ptr_eq(&node, &self.start) {
				self.cur = None;
			} else {
				self.cur = Some(node);
			}
		} else {
			self.cur = None;
		}

		Some(result)
	}
}

impl<T> LinkedNode for Rc<Node<T>> {
	type Iter = NodeIterator<T>;

	fn link(self) -> Self {
		*self.left.borrow_mut() = Rc::downgrade(&self);
		*self.right.borrow_mut() = Rc::downgrade(&self);
		*self.up.borrow_mut() = Rc::downgrade(&self);
		*self.down.borrow_mut() = Rc::downgrade(&self);

		self
	}

	fn remove_vertical(&self) {
		if let Some(ref up) = self.up.borrow().upgrade() {
			*up.down.borrow_mut() = self.down.borrow().clone();
		}

		if let Some(ref down) = self.down.borrow().upgrade() {
			*down.up.borrow_mut() = self.up.borrow().clone();
		}
	}

	fn remove_horizontal(&self) {
		if let Some(ref left) = self.left.borrow().upgrade() {
			*left.right.borrow_mut() = self.right.borrow().clone();
		}

		if let Some(ref right) = self.right.borrow().upgrade() {
			*right.left.borrow_mut() = self.left.borrow().clone();
		}
	}

	fn insert_vertical(&self) {
		if let Some(ref up) = self.up.borrow().upgrade() {
			*up.down.borrow_mut() = Rc::downgrade(self);
		}

		if let Some(ref down) = self.down.borrow().upgrade() {
			*down.up.borrow_mut() = Rc::downgrade(self);
		}
	}

	fn insert_horizontal(&self) {
		if let Some(ref left) = self.left.borrow().upgrade() {
			*left.right.borrow_mut() = Rc::downgrade(self);
		}

		if let Some(ref right) = self.right.borrow().upgrade() {
			*right.left.borrow_mut() = Rc::downgrade(self);
		}
	}

	fn append_down(&self, other: &Self) {
		*other.down.borrow_mut() = self.down.borrow().clone();
		*other.up.borrow_mut() = Rc::downgrade(self);

		if let Some(ref down) = self.down.borrow().upgrade() {
			*down.up.borrow_mut() = Rc::downgrade(other);
		}

		*self.down.borrow_mut() = Rc::downgrade(other);
	}

	fn append_right(&self, other: &Self) {
		*other.right.borrow_mut() = self.right.borrow().clone();
		*other.left.borrow_mut() = Rc::downgrade(self);

		if let Some(ref right) = self.right.borrow().upgrade() {
			*right.left.borrow_mut() = Rc::downgrade(other);
		}

		*self.right.borrow_mut() = Rc::downgrade(other);
	}

	fn append_up(&self, other: &Self) {
		*other.up.borrow_mut() = self.up.borrow().clone();
		*other.down.borrow_mut() = Rc::downgrade(self);

		if let Some(ref up) = self.up.borrow().upgrade() {
			*up.down.borrow_mut() = Rc::downgrade(other);
		}

		*self.up.borrow_mut() = Rc::downgrade(other);
	}

	fn append_left(&self, other: &Self) {
		*other.left.borrow_mut() = self.left.borrow().clone();
		*other.right.borrow_mut() = Rc::downgrade(self);

		if let Some(ref left) = self.left.borrow().upgrade() {
			*left.right.borrow_mut() = Rc::downgrade(other);
		}

		*self.left.borrow_mut() = Rc::downgrade(other);
	}

	fn iter(&self, dir: Direction) -> Self::Iter {
		NodeIterator {
			start: self.clone(),
			cur: Some(self.clone()),
			dir: dir
		}
	}
}

#[derive(Debug)]
pub struct LinkedMatrix<T: Default + Ord + Eq + Hash + Clone + ToString + fmt::Debug> {
	root: Rc<Node<T>>,
	headers: HashMap<T, Rc<Node<T>>>,
	constraints: HashMap<T, Vec<Rc<Node<T>>>>
}

impl<T: Default + Ord + Eq + Hash + Clone + ToString + fmt::Debug> fmt::Display for LinkedMatrix<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut table = prettytable::Table::new();

		let headers: Vec<Rc<Node<T>>> = self.root.iter(Direction::Right).collect();

		table.add_row(self.root.iter(Direction::Right).map(|node| cell!(format!("{}:{}:{}", node.val.to_string(), node.multiplicity.get(), node.size.get()))).collect());

		let mut rows = HashMap::new();

		for (i, header) in self.root.iter(Direction::Right).skip(1).enumerate() {
			for cell in header.iter(Direction::Down).skip(1) {
				let entry = rows.entry(cell.val.clone()).or_insert_with(|| {
					let mut row = vec![cell!(""); headers.len()];

					row[0] = cell!(cell.val);

					row
				});

				entry[i + 1] = cell!("○");
			}
		}

		let mut keys: Vec<&T> = rows.keys().collect();

		keys.sort();

		for key in keys {
			table.add_row(rows[key].iter().cloned().collect());
		}

		table.fmt(f)
	}
}

impl<T: Default + Ord + Eq + Hash + Clone + ToString + fmt::Debug> LinkedMatrix<T> {
	pub fn new() -> Self {
		LinkedMatrix {
			root: Rc::new(Node::new(T::default())).link(),
			headers: HashMap::new(),
			constraints: HashMap::new()
		}
	}

	pub fn add_column_with_multiplicity(&mut self, key: T, multiplicity: usize) {
		let node = Node::new(key.clone());

		node.multiplicity.set(multiplicity);

		let node = Rc::new(node).link();

		self.headers.insert(key, node.clone());
		self.root.append_left(&node);
	}

	pub fn add_column(&mut self, key: T) {
		self.add_column_with_multiplicity(key, 1)
	}

	pub fn add_constraint(&mut self, value: T, keys: &[T]) {
		let nodes: Vec<Rc<Node<T>>> = keys.iter().map(|k| Rc::new(Node::new_for_column(value.clone(), &self.headers[k]))).collect();

		for pair in nodes.windows(2) {
			*pair[0].right.borrow_mut() = Rc::downgrade(&pair[1]);
			*pair[1].left.borrow_mut() = Rc::downgrade(&pair[0]);
		}

		{
			let first = nodes.first().unwrap();
			let last = nodes.last().unwrap();

			*first.left.borrow_mut() = Rc::downgrade(last);
			*last.right.borrow_mut() = Rc::downgrade(first);
		}

		for node in &nodes {
			if let Some(column) = node.column.borrow_mut().upgrade() {
				column.append_up(node);
				column.size.set(column.size.get() + 1);
			}
		}

		self.constraints.insert(value, nodes);
	}

	fn cover_column(&self, column: Rc<Node<T>>) {
		column.remove_horizontal();

		for node in column.iter(Direction::Down).skip(1) {
			for cell in node.iter(Direction::Right).skip(1) {
				cell.remove_vertical();

				if let Some(col) = cell.column.borrow().upgrade() {
					col.size.set(col.size.get() - 1);
				}
			}
		}
	}

	fn uncover_column(&self, column: Rc<Node<T>>) {
		for node in column.iter(Direction::Up).skip(1) {
			for cell in node.iter(Direction::Left).skip(1) {
				cell.insert_vertical();

				if let Some(col) = cell.column.borrow().upgrade() {
					col.size.set(col.size.get() + 1);
				}
			}
		}

		column.insert_horizontal();
	}

	fn choose_column(&self) -> Option<Rc<Node<T>>> {
		let mut selected = None;
		let mut best = usize::max_value();

		for column in self.root.iter(Direction::Right).skip(1) {
			let size = column.size.get();
			let mult = column.multiplicity.get();

			if mult > size {
				return None
			}

			if size > 0 && (size - mult) < best {
				best = size - mult;
				selected = Some(column);
			}
		}

		selected
	}

	fn search(&mut self, solution: &mut Vec<T>) -> Option<Vec<T>> {
		if self.root.iter(Direction::Right).skip(1).next().is_none() {
			let mut found = solution.clone();

			found.sort();

			return Some(found);
		}

		let column = self.choose_column();

		if column.is_none() {
			return None;
		}

		let column = column.unwrap();
		let mult = column.multiplicity.get();

		column.multiplicity.set(mult - 1);

		if mult > 1 {
			column.size.set(column.size.get() - 1);
		} else {
			self.cover_column(column.clone());
		}

		for node in column.iter(Direction::Down).skip(1) {
			solution.push(node.val.clone());

			if mult > 1 {
				node.remove_vertical();
			}

			for cell in node.iter(Direction::Right).skip(1) {
				if let Some(col) = cell.column.borrow().upgrade() {
					let cellmult = col.multiplicity.get();

					col.multiplicity.set(cellmult - 1);

					if mult > 1 {
						cell.remove_vertical();
						col.size.set(col.size.get() - 1);
					}

					if cellmult == 1 {
						self.cover_column(col);
					}
				}
			}

			if let Some(found) = self.search(solution) {
				return Some(found);
			}

			for cell in node.iter(Direction::Left).skip(1) {
				if let Some(col) = cell.column.borrow().upgrade() {
					let cellmult = col.multiplicity.get();

					col.multiplicity.set(cellmult + 1);

					if cellmult == 0 {
						self.uncover_column(col.clone());
					}

					if mult > 1 {
						col.size.set(col.size.get() + 1);
						cell.insert_vertical();
					}
				}
			}

			if mult > 1 {
				node.insert_vertical();
			}

			solution.pop();
		}

		column.multiplicity.set(mult);

		if mult > 1 {
			column.size.set(column.size.get() + 1);
		} else {
			self.uncover_column(column);
		}

		None
	}

	pub fn solve(&mut self) -> Option<Vec<T>> {
		let mut solution = vec![];

		for column in self.root.iter(Direction::Right).skip(1) {
			if column.multiplicity.get() == 0 {
				self.cover_column(column);
			}
		}

		self.search(&mut solution)
	}
}
