#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        N: usize,
        L: usize,
        R: usize,
    }

    let mut ans = Vec::with_capacity(N);

    for i in 1..=N {
        if (L..=R).contains(&i) {
            let mirrored = R - (i - L);
            ans.push(mirrored);
        } else {
            ans.push(i);
        }
    }

    println!("{}", ans.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" "));
}
