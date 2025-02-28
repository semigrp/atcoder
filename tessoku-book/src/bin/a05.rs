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

fn main() {
    input! {
        N: usize,
        K: usize,
    }
        
    let mut c = 0;
    
    for s in 1..N {
        for t in 1..N {
            let u = K - (r + b);
            if (1..N).contains(&w) {
                c +=1;
            }
        }
    }
    
    println!("{}", count);
}
