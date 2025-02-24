use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars, t: Chars
    }

    let n = s.len();
    let m = t.len();

    // 編集距離(下移動は削除で右移動は追加に対応,一致するときはコスト0で斜めに移動できる.一致しないときは編集によりコスト1で斜めに移動できる)
    let mut dp: Vec<Vec<usize>> = vec![vec![1000_000_000_000_usize; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][0] = i;
    }
    for i in 1..=m {
        dp[0][i] = i;
    }

    for i in 1..=n {
        for j in 1..=m {
            // 削除か追加か
            dp[i][j] = dp[i][j - 1].min(dp[i - 1][j]) + 1;
            if s[i - 1] == t[j - 1] {
                // 無編集
                dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
            } else {
                // 編集
                dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 1);
            }
        }
    }

    println!("{}", dp[n][m])
}
