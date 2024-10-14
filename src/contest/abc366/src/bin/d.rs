use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [[[i128; n]; n];n],
        q: usize,
        query: [[usize; 6]; q],
    }

    // let m = n + 1;

    let mut dp: Vec<Vec<Vec<i128>>> = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

    // let mut cur_vol: u128 = 0;

    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                dp[i][j][k] = dp[i][j][k - 1] + dp[i][j - 1][k] + dp[i - 1][j][k]
                    - dp[i - 1][j - 1][k]
                    - dp[i][j - 1][k - 1]
                    - dp[i - 1][j][k - 1]
                    + dp[i - 1][j - 1][k - 1]
                    + a[i - 1][j - 1][k - 1];
                // println!("{}", dp[i][j][k]);
            }
        }
    }

    // let mut ans: Vec<u128> = vec![0; q];
    for i in 0..q {
        let pre_ans: i128 = dp[query[i][1]][query[i][3]][query[i][5]]
            + dp[query[i][0] - 1][query[i][2] - 1][query[i][5]]
            + dp[query[i][1]][query[i][2] - 1][query[i][4] - 1]
            + dp[query[i][0] - 1][query[i][3]][query[i][4] - 1]
            - dp[query[i][0] - 1][query[i][3]][query[i][5]]
            - dp[query[i][1]][query[i][2] - 1][query[i][5]]
            - dp[query[i][1]][query[i][3]][query[i][4] - 1]
            - dp[query[i][0] - 1][query[i][2] - 1][query[i][4] - 1];

        println!("{}", pre_ans);
        // ans[i] = pre_ans;
    }

    // for i in 0..q {
    //     println!("{}", ans[i])
    // }
}
