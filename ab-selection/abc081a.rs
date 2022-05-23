use proconio::input;

fn main() {
    input!{
        x: String,
    }
    let mut c = 0;
    for i in x.as_str().chars() {
        if i == '1' {c += 1;}
    }
    println!("{}", c);
}