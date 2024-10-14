use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, l: usize
    }

    let mut dp: Vec<u64> = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] += dp[i];
        // if i + 1 >= l {
        //     let j = i + 1 - l;
        //     dp[i + 1] += dp[j];
        // }
        dp[i + 1] += if i + 1 >= l { dp[i + 1 - l] } else { 0 };
        dp[i + 1] %= 1000000007;
    }
    println!("{}", dp[n]);
}
