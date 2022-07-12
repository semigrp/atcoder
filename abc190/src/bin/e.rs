use std::usize;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize,
        ab: [(usize, usize); M],
        K: usize,
        cd: [(usize, usize); K]
    };

    println!("{}", solve(N, M, ab, K, cd));
}

fn solve(n: usize, m: usize, ab: [(usize, usize); m], k: usize, cd: [(usize, usize); k]) -> usize {
    let mut ans = 0;
    for i in 0..(1 << k) {
        let mut cnt = vec![0; n];
        for j in 0..k {
            if i & (1 << j) != 0 {
                cnt[cd[j].0 - 1] += 1;
            } else {
                cnt[cd[j].1 - 1] += 1;
            }
        }
        let mut tot = 0;
        for j in 0..m {
            if cnt[ab[j].0 - 1] > 0 && cnt[ab[j].1 - 1] > 0 {
                tot += 1;
            }
        }
        ans = ans.max(tot);
    }
    ans
}
