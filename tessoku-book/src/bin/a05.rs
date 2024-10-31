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

fn solve(n: i32, k:i32) -> Result<i32> {
	let mut result = 0;
	for i in 1..n+1 {
		for j in 1..n+1 {
			if k - (i + j) <= n && k - (i + j) > 0 {
				result += 1;
			}
		}
	}
    Ok(result)
}

fn main() -> Result<()> {
    input! {
	n: i32,
	k: i32,
    }
    let result = solve(n, k)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(3, 6).unwrap(), 7);
    }
}
