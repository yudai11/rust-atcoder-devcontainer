use proconio::{input, marker::Chars};

// LCS(最長共通部分列)
fn main() {
    input! {
        s: Chars, t: Chars
    }

    let n = s.len();
    let m = t.len();

    let mut dp = vec![vec![0_usize; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }

    println!("{}", dp[n][m])
}
