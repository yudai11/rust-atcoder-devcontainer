use itertools::Itertools;
use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        t: usize
    }

    let mut ans = vec![];

    for _i in 0..t {
        input! {
            n: usize,
            s: Chars
        }

        // i番目まで見てすでに1を見逃したか(現在見ているか)とこのマスが0/1のどちらかを指定した上でルールを満たす最小コストを記録(unseen,0)(1)(seen,0)とする。
        let mut dp = vec![vec![0_usize; 3]; n];

        match s[0] {
            '0' => {
                dp[0][1] = 1;
            }
            '1' => {
                dp[0][0] = 1;
            }
            _ => unreachable!(),
        }
        for i in 1..n {
            match s[i] {
                '0' => {
                    dp[i][0] = dp[i - 1][0];
                    dp[i][1] = (dp[i - 1][0] + 1).min(dp[i - 1][1] + 1);
                    dp[i][2] = dp[i - 1][2].min(dp[i - 1][1]);
                }
                '1' => {
                    dp[i][0] = dp[i - 1][0] + 1;
                    dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]);
                    dp[i][2] = (dp[i - 1][2] + 1).min(dp[i - 1][1] + 1);
                }
                _ => unreachable!(),
            }
        }

        let res = dp[n - 1][0].min(dp[n - 1][1].min(dp[n - 1][2]));
        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"));
}
