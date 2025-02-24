use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
        lr: [(Usize1,Usize1); q]
    }

    let mut cum_sum = vec![0_usize; n];
    for i in 1..n {
        if s[i] == s[i - 1] {
            cum_sum[i] = cum_sum[i - 1] + 1;
        } else {
            cum_sum[i] = cum_sum[i - 1];
        }
    }

    let mut ans = vec![];

    for &(l, r) in lr.iter() {
        let mut res = cum_sum[r];
        if l == 0 {
            ans.push(res);
        } else if s[l] == s[l - 1] {
            res -= cum_sum[l];
            ans.push(res);
        } else {
            res -= cum_sum[l - 1];
            ans.push(res);
        }
    }

    // 入出力はまとめたほうが速い
    println!("{}", ans.iter().join("\n"));
}
