use proconio::input;

fn main() {
    input! {
        h: usize,//行
        w: usize,//列
        mut a: [[i32; w];h]
    }

    let mut r = vec![0; h];
    let mut c = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            row[i] += a[i][j];
            col[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", r[i] + c[j] - a[i][j]);
        }
        println!();
    }
}
