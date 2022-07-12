use std::usize;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {n:usize};

    let ans = enum_divisors(n);

    println!("{}", ans.iter().filter(|x| *x % 2 == 1).count() * 2);
}

pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i as usize);
            if i * i != n {
                res.push(n / i as usize);
            }
        }
        i += 1;
    }
    res
}
