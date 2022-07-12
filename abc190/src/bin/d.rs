use std::usize;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {n:i128};

    let ans = enum_divisors(n);

    println!("{}", ans.iter().filter(|x| *x % 2 == 1).count() * 2);
}

pub fn enum_divisors(n: i128) -> Vec<usize> {}
