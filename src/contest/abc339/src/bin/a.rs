use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    let mut ans = vec![];
    // let mut last_period = 0_usize;
    for i in (0..n).rev() {
        if s[i] == '.' {
            break;
        }
        ans.push(s[i]);
    }
    ans.reverse();
    println!("{}", ans.iter().join(""));
}
