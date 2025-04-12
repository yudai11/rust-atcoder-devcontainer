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
        n: usize, k: usize,
        s: Chars
    }

    let mut t = s.clone();

    // i文字目をj(./o)としたときのmin_maxを記録
    let mut dp = vec![vec![vec![0_usize; 2]; 2]; n + 1];
    for i in 0..n {
        match s[i] {
            '.' => {
                dp[i + 1][0][0] = dp[i][0][0].min(dp[i][1][0]);
                dp[i + 1][1][0] = dp[i][0][0].min(dp[i][1][0]);
                dp[i + 1][0][1] = dp[i][1][1].max(dp[i][0][1]);
                dp[i + 1][1][1] = dp[i][1][1].max(dp[i][0][1]);
            }
            'o' => {
                dp[i + 1][0][0] = dp[i][0][0].min(dp[i][1][0]) + 1;
                dp[i + 1][1][0] = dp[i][0][0].min(dp[i][1][0]) + 1;
                dp[i + 1][0][1] = dp[i][1][1].max(dp[i][0][1]) + 1;
                dp[i + 1][1][1] = dp[i][1][1].max(dp[i][0][1]) + 1;
            }
            _ => {
                if (i > 0 && s[i - 1] == 'o') || (i < n - 1 && s[i + 1] == 'o') {
                    t[i] = '.';
                    dp[i + 1][0][0] = dp[i][0][0].min(dp[i][1][0]);
                    dp[i + 1][1][0] = dp[i][0][0].min(dp[i][1][0]);
                    dp[i + 1][0][1] = dp[i][1][1].max(dp[i][0][1]);
                    dp[i + 1][1][1] = dp[i][1][1].max(dp[i][0][1]);
                } else if i > 0 && s[i - 1] == '?' {
                    dp[i + 1][1][1] = dp[i][0][1] + 1;
                    dp[i + 1][0][1] = (dp[i][1][1]).max(dp[i][0][1]);
                    dp[i + 1][0][0] = dp[i][0][0];
                    dp[i + 1][1][0] = dp[i][0][0] + 1;
                } else {
                    // 1つ前は'.'
                    dp[i + 1][1][1] = dp[i][0][1] + 1;
                    dp[i + 1][0][1] = (dp[i][1][1]).max(dp[i][0][1]);
                    dp[i + 1][0][0] = dp[i][0][0];
                    dp[i + 1][1][0] = dp[i][0][0] + 1;
                }
            }
        }
    }

    // 逆順に見る
    let mut dp_rev = vec![vec![vec![0_usize; 2]; 2]; n + 1];
    for i in (0..n).rev() {
        match s[i] {
            '.' => {
                dp_rev[i][0][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]);
                dp_rev[i][1][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]);
                dp_rev[i][0][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]);
                dp_rev[i][1][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]);
            }
            'o' => {
                dp_rev[i][0][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]) + 1;
                dp_rev[i][1][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]) + 1;
                dp_rev[i][0][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]) + 1;
                dp_rev[i][1][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]) + 1;
            }
            _ => {
                if (i > 0 && s[i - 1] == 'o') || (i < n - 1 && s[i + 1] == 'o') {
                    t[i] = '.';
                    dp_rev[i][0][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]);
                    dp_rev[i][1][0] = dp_rev[i + 1][0][0].min(dp_rev[i + 1][1][0]);
                    dp_rev[i][0][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]);
                    dp_rev[i][1][1] = dp_rev[i + 1][1][1].max(dp_rev[i + 1][0][1]);
                } else if i < n - 1 && s[i + 1] == '?' {
                    dp_rev[i][1][1] = dp_rev[i + 1][0][1] + 1;
                    dp_rev[i][0][1] = (dp_rev[i + 1][1][1]).max(dp_rev[i + 1][0][1]);
                    dp_rev[i][0][0] = dp_rev[i + 1][0][0];
                    dp_rev[i][1][0] = dp_rev[i + 1][0][0] + 1;
                } else {
                    // 1つ前は'.'
                    dp_rev[i][1][1] = dp_rev[i + 1][0][1] + 1;
                    dp_rev[i][0][1] = (dp_rev[i + 1][1][1]).max(dp_rev[i + 1][0][1]);
                    dp_rev[i][0][0] = dp_rev[i + 1][0][0];
                    dp_rev[i][1][0] = dp_rev[i + 1][0][0] + 1;
                }
            }
        }
    }

    for i in 0..n {
        if t[i] != '?' {
            continue;
        }

        let mut can_dot = true;
        let mut can_cir = true;

        if dp[i + 1][0][0] + dp_rev[i][0][0] > k || dp[i + 1][0][1] + dp_rev[i][0][1] < k {
            can_dot = false;
        }
        if dp[i + 1][1][0] + dp_rev[i][1][0] - 1 > k || dp[i + 1][1][1] + dp_rev[i][1][1] - 1 < k {
            can_cir = false;
        }

        if !can_cir {
            t[i] = '.'
        }
        if !can_dot {
            t[i] = 'o';
        }
    }

    println!("{}", t.iter().join(""));
}
