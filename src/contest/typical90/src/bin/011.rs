use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut dcs: [(Usize1,Usize1,usize); n]
    }

    // 締め切り順にソートしてからdp
    dcs.sort_by_key(|x| x.0);

    // i個目までの仕事からいくつか選んでj日までに受け取った報酬の最大値
    let mut dp = vec![vec![0_usize; 5000]; n + 1];
    for (i, &(d, c, s)) in dcs.iter().enumerate() {
        for j in 0..c {
            dp[i + 1][j] = dp[i][j];
        }
        for j in c..=d {
            // (j - c) ~ j まで仕事を行う．-> (j - c - 1) 日目での報酬 + s
            if j > c {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - c - 1] + s);
            } else {
                dp[i + 1][j] = dp[i][j].max(s);
            }
        }
        for j in d + 1..5000 {
            dp[i + 1][j] = dp[i][j].max(dp[i + 1][j - 1])
        }
    }

    println!("{}", dp[n][4999]);
}
