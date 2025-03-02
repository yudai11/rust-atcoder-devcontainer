use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cp: [(Usize1,usize); n],
        q: usize,
        lr: [(Usize1,Usize1); q]
    }

    // 組jにおける学籍番号0~iの生徒の合計得点
    let mut sum_score = vec![vec![0_usize; 2]; n];
    for (i, &(c, p)) in cp.iter().enumerate() {
        if i > 0 {
            sum_score[i][0] = sum_score[i - 1][0];
            sum_score[i][1] = sum_score[i - 1][1];
        }
        sum_score[i][c] += p;
    }

    let mut ans = vec![vec![]; q];
    for (i, &(l, r)) in lr.iter().enumerate() {
        if l == 0 {
            ans[i].push(sum_score[r][0]);
            ans[i].push(sum_score[r][1]);
        } else {
            ans[i].push(sum_score[r][0] - sum_score[l - 1][0]);
            ans[i].push(sum_score[r][1] - sum_score[l - 1][1]);
        }
    }

    println!("{}", ans.iter().map(|ai| ai.iter().join(" ")).join("\n"));
}
