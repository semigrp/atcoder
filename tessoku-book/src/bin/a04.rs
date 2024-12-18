#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Result;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
 
fn solve(n: i32) -> Result<String> {
    let mut m = n;
    let mut result = 0;
    let mut i = 0;
    while m >= 2 {
        result += (m % 2) * 10_i32.pow(i); 
        m /= 2;
        i += 1;
    } 
    result += m * 10_i32.pow(i);
    Ok(format!("{:010}", result))
}

fn main() -> Result<()> {
    input! {
        n:i32,
    }
    let result = solve(n)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(5).unwrap(), "0000000101");
    }
}
