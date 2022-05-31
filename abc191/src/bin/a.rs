use proconio::input;

fn main() {
    input! {
        v: i32,
        t: i32,
        s: i32,
        d: i32,
    }

    if d < v * t || d > v * s {
        println!("Yes");
    } else {
        println!("No");
    }
}
