#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(usize, usize); N],
    }

    let mut dsums = vec![0i32; D + 2];
    
    for &(l, r) in &LR { 
        dsums[l] += 1;
        dsums[r + 1] -= 1;
    }

    for i in 1..=D {
        dsums[i] += dsums[i - 1];
        println!("{}", dsums[i]);
    }

}
