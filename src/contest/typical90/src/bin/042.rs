use proconio::input;

fn main() {
    input! {
        k: usize
    }

    const MOD: usize = 1000_000_007;

    if k % 9 != 0 {
        println!("0");
        return;
    }

    // 桁和が i mod Kとなる正整数の数 (online)
    let mut dp = vec![0_usize; 10];
    dp[0] = 1;
    for i in 1..=k {
        // let l = i % 10;
        // 桁和が j 小さい正整数の頭に j in [1..9]を追加することで桁和が i の正整数ができる．逆に全ての桁和が i の正整数はこの操作で作れる(全単射)
        for j in 1..9 {
            dp[i % 9] += dp[(i + j) % 9];
        }
        dp[i % 9] %= MOD;
    }

    println!("{}", dp[0]);
}
