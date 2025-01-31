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

fn main() -> Result<()> {
    input! {
    H: usize,
    W: usize,
    grid: [[i64; W]; H],
    Q: usize,
    queries: [(usize, usize, usize, uszie); Q]
    }
    
    let mut S = vec![vec![0; W + 1]; H + 1];
    
    for i in 1..=H {
        for j in 1..=W {
            S[i][j] = grid[i - 1][j - 1] + S[i - 1][j] + S[i][j - 1] - S[i - 1][j - 1];
        }
    }
    
    let mut results = Vec::with_capacity(Q);
    for &(A, B, C, D) in &queries {
        let sum = S[C][D] - S[A - 1][D] - S[C][B - 1] + S[A - 1][B - 1];
        results.push(sum);
    }
    
    for res in results {
        println!("{}", res);
    }
    
    Ok(())
}
