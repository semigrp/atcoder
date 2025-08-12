#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let ans = 1000 - 100 * N;

    println!("{}", ans);
} 
