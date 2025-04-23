#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque, HashMap, HashSet};
use std::cmp::{min, max};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1_000_000_007;
const INF: usize = 1 << 60;

fn main() {
  solve();
}

#[fastout]
fn solve() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize); q]
    }

    let mut cumsum = vec![0i64; n+1];
    for i in 0..n {
        cumsum[i+1] = cumsum[i] + a[i];
    }

    for (l, r) in queries {
        let sum = cumsum[r] - cumsum[l-1];
        println!("{}", sum);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}

fn mod_add(a: usize, b: usize) -> usize {
    (a + b) % MOD
}

fn mod_mul(a: usize, b: usize) -> usize {
    ((a % MOD) * (b % MOD)) % MOD
}

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
