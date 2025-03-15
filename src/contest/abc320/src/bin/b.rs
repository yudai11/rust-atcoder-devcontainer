use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let l = s.len();
    // 文字数iでjから始まる回文が存在するか
    let mut dp = vec![vec![0_usize; l]; l + 1];
    for i in 0..l {
        dp[0][i] = 1;
        dp[1][i] = 1;
    }

    for i in 2..=l {
        for j in 0..=l - i {
            if s[j] == s[j + i - 1] && dp[i - 2][j + 1] > 0 {
                dp[i][j] = i;
            }
        }
    }

    let ans = dp
        .iter()
        .map(|x| x.iter().fold(0_usize, |a, &y| a.max(y)))
        .fold(0_usize, |b, z| b.max(z));

    println!("{}", ans);
}
