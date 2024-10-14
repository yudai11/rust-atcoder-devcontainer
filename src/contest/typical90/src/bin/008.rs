use proconio::input;
use proconio::marker::Chars;
// use itertools::Itertools;

const MOD: u64 = 1000000007;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut dp = vec![vec![0; 8]; n + 1];
    for i in 0..n {
        dp[i][0] = 1;
    }

    for i in 0..n {
        for j in 1..8 {
            dp[i + 1][j] = dp[i][j];
        }
        let si = s[i];
        let is_in_atcoder: Result<usize, usize> = is_included_in_atcoder(si);
        match is_in_atcoder {
            Ok(num) => {
                dp[i + 1][num] += dp[i][num - 1];
                dp[i + 1][num] %= MOD;
            }
            Err(_) => {}
        }
    }

    println!("{}", dp[n][7]);
}

fn is_included_in_atcoder(si: char) -> Result<usize, usize> {
    let v = "atcoder";
    for (i, vi) in v.chars().enumerate() {
        if vi == si {
            // print!("{} ", i + 1);
            return Ok(i + 1);
        }
    }
    return Err(0);
}
