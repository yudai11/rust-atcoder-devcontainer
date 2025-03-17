use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, s: usize,
        ab: [(usize,usize); n]
    }

    // i 個目の商品まで選んで j 円にする方法があるか
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    let mut up_bound = 0_usize;
    for i in 1..=n {
        let (a, b) = ab[i - 1];
        let m = up_bound;
        for j in 0..=m {
            if !dp[i - 1][j] {
                continue;
            }
            if j + a <= s {
                dp[i][j + a] = true;
                up_bound = up_bound.max(j + a);
            }
            if j + b <= s {
                dp[i][j + b] = true;
                up_bound = up_bound.max(j + b);
            }
            if j + a > s && j + b > s {
                break;
            }
        }
    }

    if !dp[n][s] {
        println!("Impossible");
        return;
    }

    // dp の復元
    let mut ans = vec![];
    let mut paid = s;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        if a <= paid && dp[i][paid - a] {
            ans.push('A');
            paid -= a;
        } else {
            ans.push('B');
            paid -= b;
        }
    }

    ans.reverse();

    println!("{}", ans.iter().join(""));
}
