use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let c = a * b;

    if c % 2 == 0 {
        println!("{}", "Even");
    }else {
        println!("{}", "Odd");
    }
}