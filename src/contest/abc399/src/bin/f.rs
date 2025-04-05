use proconio::input;

const MOD: u128 = 998244353;

fn main() {
    input! {
        n: usize, k: usize,
        a: [u128; n]
    }

    let mut binom = vec![vec![0_u128; k + 1]; k + 1];
    for i in 0..=k {
        binom[i][0] = 1;
        for j in 1..=i {
            binom[i][j] = binom[i - 1][j] + binom[i - 1][j - 1];
            binom[i][j] %= MOD;
        }
    }

    let mut dp = vec![vec![0_u128; k + 1]; n + 1];
    dp[0][0] = 1;
    let mut ans = 0_u128;

    for i in 0..n {
        let mut power_ai = vec![1_u128; k + 1];
        for j in 0..k {
            power_ai[j + 1] = power_ai[j] * a[i];
            power_ai[j + 1] %= MOD;
        }

        dp[i + 1][0] = dp[i][0] + 1;
        dp[i + 1][0] %= MOD;
        for j in 1..=k {
            for l in 0..=j {
                dp[i + 1][j] += dp[i][j - l] * binom[j][j - l] * power_ai[l];
                dp[i + 1][j] %= MOD;
            }
        }

        ans += dp[i + 1][k];
        ans %= MOD;
    }

    println!("{}", ans);
}
