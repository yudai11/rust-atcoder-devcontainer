use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        k: usize
    }
    let mod_int: usize = 1000000007;

    if k % 9 != 0 {
        println! {"0"};
    } else {
        // iは桁数
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 1..=k {
            for j in 1..=((i).min(9)) {
                dp[i] += dp[i - j];
                dp[i] %= MOD;
            }
        }

        println!("{}", dp[k]);
    }
}
