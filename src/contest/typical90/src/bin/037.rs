use std::collections::VecDeque;
// use std::process::exit;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize,usize,i64);n]
    }

    let mut dp = vec![i64::MIN; w + 1];
    dp[0] = 0;

    let mut deq: VecDeque<(i64, usize)> = VecDeque::new();

    for &(l, r, v) in lrv.iter() {
        let mut tmp_dp = dp.clone();
        // empty deq
        deq.clear();

        // Search for only the extent to which dishes can be added can be updated.
        for i in 0..=w - l {
            if deq.len() > 0 && deq[0].1 == i {
                deq.pop_front();
            }
            if !deq.is_empty() && deq[0].0 <= dp[i] {
                deq.clear();
            }
            deq.push_back((dp[i], i + r + 1 - l));
            let cv = deq[0].0;
            tmp_dp[i + l] = tmp_dp[i + l].max(cv + v);
        }

        dp = tmp_dp;
    }

    let mut res = dp[w];
    if res < 0 {
        res = -1;
    }
    println!("{}", res);
}
