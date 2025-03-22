use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];

    if n % 2 != 0 {
        for i in 0..n / 2 {
            ans.push('-');
        }
        ans.push('=');
        for i in 0..n / 2 {
            ans.push('-');
        }
    } else {
        for i in 1..n / 2 {
            ans.push('-');
        }
        ans.push('=');
        ans.push('=');
        for i in 1..n / 2 {
            ans.push('-');
        }
    }

    println!("{}", ans.iter().join(""));
}
