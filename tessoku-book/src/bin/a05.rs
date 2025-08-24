#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{fastout, input};


#[fastout]
fn main() {
    input! {
        N: i64,
        K: i64,
    }

    let mut ans: i64 = 0;

    for i in 1..=N{
        for j in 1..=N {
            let w = K - i - j;
            if 1 <= w && w <= N {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
