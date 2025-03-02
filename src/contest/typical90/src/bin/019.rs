use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; 2 * n]
    }

    //  区間dp
    let mut dp = vec![vec![2000_000_000_usize; 2 * n]; 2 * n];
    for i in 0..2 * n - 1 {
        dp[i][i + 1] = (a[i] - a[i + 1]).abs() as usize;
    }

    for gap in (3..2 * n).step_by(2) {
        for j in 0..2 * n {
            if j + gap >= 2 * n {
                break;
            }
            for k in (j + 1..j + gap - 1).step_by(2) {
                dp[j][j + gap] = dp[j][j + gap].min(dp[j][k] + dp[k + 1][j + gap]);
            }
            dp[j][j + gap] =
                dp[j][j + gap].min(dp[j + 1][j + gap - 1] + (a[j + gap] - a[j]).abs() as usize);
        }
    }

    println!("{}", dp[0][2 * n - 1]);
}
