use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    const MOD: usize = 1000_000_007;

    // i文字目まで見たとき"atcoder"のj文字目まで作れる選び方がいくつあるか
    let mut dp = vec![vec![0_usize; 8]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..8 {
            dp[i + 1][j] = dp[i][j];
        }
        match s[i] {
            'a' => {
                dp[i + 1][1] += 1;
            }
            't' => {
                dp[i + 1][2] += dp[i][1];
            }
            'c' => {
                dp[i + 1][3] += dp[i][2];
            }
            'o' => {
                dp[i + 1][4] += dp[i][3];
            }
            'd' => {
                dp[i + 1][5] += dp[i][4];
            }
            'e' => {
                dp[i + 1][6] += dp[i][5];
            }
            'r' => {
                dp[i + 1][7] += dp[i][6];
            }
            _ => {}
        }
        for j in 0..8 {
            dp[i + 1][j] %= MOD;
        }
    }

    println!("{}", dp[n][7]);
}
