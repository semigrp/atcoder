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

fn solve(n: usize, q: usize, a: Vec<usize>, lr: Vec<(usize, usize) >) -> Result<String> {
    let mut prefix_sum = vec![0; n+1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + a[i];
    }
    
    let mut results = Vec::with_capacity(q);
    for &(L, R) in &lr {
        let sum = prefix_sum[R] - prefix_sum[L - 1];
        results.push(sum);
    }  
    Ok(results.iter().join("\n"))
}

fn main() -> Result<()> {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let result = solve(n, q, a, lr)?;
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
