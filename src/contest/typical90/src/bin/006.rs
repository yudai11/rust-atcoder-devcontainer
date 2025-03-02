use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, k: usize,
        s: Chars
    }

    // 位置i以降でalphabet j が現れる最左の位置を記録
    let mut dp = vec![vec![n; 26]; n];
    for i in (0..n).rev() {
        let si = s[i] as usize - 'a' as usize;
        for j in 0..26 {
            if j == si {
                dp[i][j] = i;
            } else if i < n - 1 {
                dp[i][j] = dp[i + 1][j];
            }
        }
    }

    // 貪欲にalphabetを決める
    let mut ans = String::from("");
    let mut last = 0_usize;
    for i in 0..k {
        for j in 0..26 {
            let alph = (j as u8 + 'a' as u8) as char;
            let next = dp[last][j];
            if n - next >= k - i {
                last = next + 1;
                ans.push(alph);
                break;
            }
        }
    }

    println!("{}", ans);
}
