use proconio::{input, marker::Usize1};

// 区間DP
fn main() {
    input! {
        n: usize,
        scores: [(Usize1,usize);n]
    }

    let mut dp = vec![vec![0_usize; n]; n];

    // 区間幅が大きい順に上から処理
    for len in (0..=n - 2).rev() {
        for left in 0..n - len {
            let right = left + len;
            if left > 0 {
                let score_l = if right >= scores[left - 1].0 && left <= scores[left - 1].0 {
                    scores[left - 1].1
                } else {
                    0
                };
                dp[left][right] = dp[left][right].max(dp[left - 1][right] + score_l);
            }
            if right < n - 1 {
                let score_r = if right >= scores[right + 1].0 && left <= scores[right + 1].0 {
                    scores[right + 1].1
                } else {
                    0
                };
                dp[left][right] = dp[left][right].max(dp[left][right + 1] + score_r);
            }
        }
    }

    let mut ans = 0_usize;
    for i in 0..n {
        ans = ans.max(dp[i][i]);
    }

    println!("{}", ans);

    // println!("{}", dp.iter().map(|si| si.iter().join("")).join("\n"));
}
