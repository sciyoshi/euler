#![feature(inclusive_range_syntax, step_by, conservative_impl_trait)]

#[macro_use]
extern crate num;
extern crate primal;
extern crate roaring;

mod problem60;
mod problem75;
mod problem78;
mod problem91;
mod problem93;
mod problem94;
mod problem95;

fn main() {
	println!("Euler #95: {}", problem95::solve());

	if false {
		println!("Euler #60: {}", problem60::solve());
		println!("Euler #75: {}", problem75::solve());
		println!("Euler #78: {}", problem78::solve());
		println!("Euler #91: {}", problem91::solve());
		println!("Euler #93: {}", problem93::solve());
		println!("Euler #94: {}", problem94::solve());
	}
}
