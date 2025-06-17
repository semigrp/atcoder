#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        A: i32,
        B: i32,
    }

    let result = A + B;

    println!("{}", result);
}
