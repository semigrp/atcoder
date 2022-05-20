use std::io;

fn main() {
  let x = get_input();
  println!("{}", x * 2);
}

fn get_input() -> i32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}