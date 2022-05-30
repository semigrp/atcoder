use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if c == 0 {
        if b < a {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        if a < b {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
