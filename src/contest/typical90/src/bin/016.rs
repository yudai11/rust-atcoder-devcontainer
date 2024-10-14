use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
    }

    let mut ans = 10000;

    for i in 0..10000 {
        for j in 0..(10000 - i) {
            if a * i + b * j > n {
                break;
            }
            if (n - a * i - b * j) % c == 0 {
                ans = ans.min(i + j + (n - a * i - b * j) / c)
            }
        }
    }

    println!("{}", ans);
}
