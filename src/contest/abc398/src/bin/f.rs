use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    // $ はダミー文字
    let mut ms = vec!['$'];
    for &si in s.iter() {
        ms.push(si);
        ms.push('$');
    }

    let r = manacher(&ms);
    let mut k = 0_usize;

    let mut i = n;
    loop {
        if r[i] + k >= n {
            break;
        }
        k += 1;
        i += 1;
    }

    let mut t = s.clone();

    for i in (0..k).rev() {
        t.push(s[i]);
    }

    println!("{}", t.iter().join(""));
}

fn manacher(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    // i を中心とする最長の回文の半径を保存
    let mut dp = vec![0_usize; n];

    let mut i = 0_usize;
    let mut j = 0_usize;

    while i < n {
        while i >= j && i + j < n && s[i - j] == s[i + j] {
            j += 1;
        }
        dp[i] = j;

        let mut k = 1_usize;
        while i >= k && i + k < n && k + dp[i - k] < j {
            dp[i + k] = dp[i - k];
            k += 1;
        }

        i += k;
        j -= k;
    }
    return dp;
}
