#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let mut max_count = 0;
    for c in s.chars() {
        if "ACGT".contains(c) {
            count += 1;
            max_count = max_count.max(count);
        } else {
            count = 0;
        }
    }

    println!("{}", max_count);

}
