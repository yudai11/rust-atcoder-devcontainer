use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, x: usize,
        vac: [(Usize1,usize,usize);n]
    }

    // 各ビタミンについて消費カロリー固定化での最大摂取量を求める(オンラインdpを用いる)
    let mut dp: Vec<Vec<usize>> = vec![vec![0_usize; x + 1]; 3];
    let (v, a, c) = vac[0];
    dp[v][c] = a;
    // online dpの実行
    for i in 1..n {
        let (v, a, c) = vac[i];
        // 小さいものを参照する関係上上から順に見ることでオンラインにdpが行える．
        for j in (c..=x).rev() {
            dp[v][j] = dp[v][j].max(dp[v][j - c] + a);
        }
    }

    // 半分全列挙によりminmaxを求める．
    // costがp以下のときのvitamin_0 とvitamin_1のminmax
    let mut minmax_zero_one = vec![0_usize; x + 1];
    for i in 0..=x {
        for j in 0..=x - i {
            minmax_zero_one[i + j] = minmax_zero_one[i + j].max(dp[0][i].min(dp[1][j]))
        }
    }
    // costがp以下のときのvitami3種類のminmax
    let mut minmax_all = vec![0; x + 1];
    for i in 0..=x {
        for j in 0..=x - i {
            minmax_all[i + j] = minmax_all[i + j].max(minmax_zero_one[i].min(dp[2][j]))
        }
    }

    let ans = minmax_all[x];
    println!("{}", ans);
}
