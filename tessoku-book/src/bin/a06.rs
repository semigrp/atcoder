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
    fn test_case_1() {
        let n = 15;
        let q = 3;
        let a = vec![62, 65, 41, 13, 20, 11, 18, 44, 53, 12, 18, 17, 14, 10, 39];
        let lr = vec![(4, 13), (3, 10), (2, 15)];
        let expected = "220\n212\n375";
        let result = solve(n, q, a, lr).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2_single_element() {
        let n = 1;
        let q = 1;
        let a = vec![42];
        let lr = vec![(1, 1)];
        let expected = "42";
        let result = solve(n, q, a, lr).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3_entire_range() {
        let n = 5;
        let q = 1;
        let a = vec![1, 2, 3, 4, 5];
        let lr = vec![(1, 5)];
        let expected = "15";
        let result = solve(n, q, a, lr).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_4_multiple_queries() {
        let n = 6;
        let q = 2;
        let a = vec![10, 20, 30, 40, 50, 60];
        let lr = vec![(1, 3), (4, 6)];
        let expected = "60\n150";
        let result = solve(n, q, a, lr).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_5_large_numbers() {
        let n = 3;
        let q = 1;
        let a = vec![1_000_000_000, 1_000_000_000, 1_000_000_000];
        let lr = vec![(1, 3)];
        let expected = "3000000000";
        let result = solve(n, q, a, lr).unwrap();
        assert_eq!(result, expected);
    }
}
