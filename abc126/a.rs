use std::ascii::AsciiExt;
 
fn main() {
    let (_, K) = read!(usize, usize);
    let mut S = rl().chars().collect::<Vec<_>>();
    S[K - 1].make_ascii_lowercase();
    println!("{}", S.into_iter().collect::<String>())
}