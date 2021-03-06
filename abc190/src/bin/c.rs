use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        dishes: [(usize, usize); m],
        k: usize,
        people: [(usize, usize); k],
    }

    let mut ans = 0;

    for i in 0..(1 << k) {
        let mut cnt = vec![0; n];
        for j in 0..k {
            if i & (1 << j) != 0 {
                cnt[people[j].0 - 1] += 1;
            } else {
                cnt[people[j].1 - 1] += 1;
            }
        }

        let mut tot = 0;

        for j in 0..m {
            if cnt[dishes[j].0 - 1] > 0 && cnt[dishes[j].1 - 1] > 0 {
                tot += 1;
            }
        }

        ans = ans.max(tot);
    }

    println!("{}", ans);
}
