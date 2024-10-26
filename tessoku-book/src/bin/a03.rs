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

fn solve(k: i32, p: &[i32], q: &[i32]) -> Result<String> {
    for &i in p {
        for &j in q {
            if j == k - p {
                return Ok("Yes".into())
            }
        }
    }  
    Ok("No".into())
}

fn main() -> Result<()> {
    input! {
        n: usize,
        k: i32,
        p: [i32; n],
        q: [i32; n],
    }
    let result = solve(k, &p, &q)?;
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
