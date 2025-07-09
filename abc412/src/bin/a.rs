#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let ans = ab.iter()
                .filter(|&&(a, b)| b > a)
                .count();

    println!("{}", ans);
}
