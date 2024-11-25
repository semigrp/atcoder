#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Result;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;

fn solve(n: usize, d: usize, lr: Vec<(usize, usize)>) -> Result<Vec<usize>> {
}

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n], 
    }
	let mut ac = vec![0; D + 2];

	for (l, r) in lr {
		ac[l] += 1;
		ac[r + 1] -= 1;
	}

	let mut ca = 0;
	for day in 1..=d {
		ca += ac[day];
		println!("{}", ca);
	}
}


