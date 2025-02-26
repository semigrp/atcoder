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
        N: usize,
        K: usize,
        P: [usize; N],
        Q: [usize; N]
    }
    
    for &p in &P {
        for &q in &Q {
            if K - p == q {
                println!("Yes");
                return;
            }
        }
    }
    
    println!("No");
    Ok(())
}
