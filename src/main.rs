#![feature(inclusive_range_syntax, step_by, conservative_impl_trait, ptr_eq)]

#[macro_use]
extern crate num;
extern crate time;
extern crate primal;
extern crate roaring;
extern crate mod_pow;
extern crate bit_vec;
extern crate bit_set;
#[macro_use]
extern crate ndarray;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate prettytable;

#[macro_use]
mod utils;
mod problem60;
mod problem75;
mod problem78;
mod problem91;
mod problem93;
mod problem94;
mod problem95;
mod problem108;
mod problem110;
mod problem111;
mod problem119;
mod problem123;
mod problem124;
mod problem133;
mod problem145;
mod problem149;
mod problem162;
mod problem167;
mod problem185;
mod problem186;
mod problem305;
mod problem347;
mod problem353;
mod problem376;

fn main() {
	println!("Euler #353: {}", problem353::solve());

	// problems in progress
	if false {
		println!("Euler #167: {}", problem167::solve());
		println!("Euler #376: {}", problem376::solve());
	}

	// completed problems
	if false {
		println!("Euler #60: {}", problem60::solve());
		println!("Euler #75: {}", problem75::solve());
		println!("Euler #78: {}", problem78::solve());
		println!("Euler #91: {}", problem91::solve());
		println!("Euler #93: {}", problem93::solve());
		println!("Euler #94: {}", problem94::solve());
		println!("Euler #95: {}", problem95::solve());
		println!("Euler #108: {}", problem108::solve());
		println!("Euler #110: {}", problem110::solve());
		println!("Euler #111: {}", problem111::solve());
		println!("Euler #119: {}", problem119::solve());
		println!("Euler #124: {}", problem124::solve());
		println!("Euler #123: {}", problem123::solve());
		println!("Euler #133: {}", problem133::solve());
		println!("Euler #145: {}", problem145::solve());
		println!("Euler #149: {}", problem149::solve());
		println!("Euler #162: {}", problem162::solve());
		println!("Euler #185: {}", problem185::solve());
		println!("Euler #186: {}", problem186::solve());
		println!("Euler #305: {}", problem305::solve());
		println!("Euler #347: {}", problem347::solve());
	}
}
