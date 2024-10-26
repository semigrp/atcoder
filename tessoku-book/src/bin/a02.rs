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

fn solve(x: i32, a: &[i32]) -> Result<String> {
    for &i in a {
        if i == x {
            Ok("Yes".into())
        }
    }
    Ok("No".into())
}

fn main() -> Result<()> {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }
    let result = solve(x, &a)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve().unwrap(), "result");
    }
}
