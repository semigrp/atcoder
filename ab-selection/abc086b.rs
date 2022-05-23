use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }
    let mut b = 0;
    let mut ans = 0;

    for i in 0..n {
        b = v[i] | b;
    }

    while (b & 1) == 0 {
        b = b >> 1;
        ans += 1;
    }

    println!("{}", ans);
}
