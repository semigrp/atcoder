#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [i32; N],
        LR: [(i32, i32); Q],
    }

    let mut sums = vec![0; N + 1];

    for i in 0..N {
        sums[i + 1] = sums[i] + A[i];
    }

    for &(l, r) in &LR {
        let result = sums[r as usize] - sums[(l - 1) as usize];
        println!("{}", result);
    }
}
