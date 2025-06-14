use itertools::Itertools;
use proconio::input;
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

const MOD: usize = 998244353;

fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t]
    }

    let mut ans = vec![];

    for &(n, k) in nk.iter() {
        let mut m = 0_usize;
        let mut x = n;
        while x > 0 {
            m += 1;
            x /= 2;
        }
        if m < k {
            ans.push(0_usize);
            continue;
        }

        let mut pre_ans = 0_usize;

        for p in (0..m).combinations(k) {
            let mut x = 0_usize;
            for i in p {
                x += 1 << i;
                x %= MOD;
            }

            if x <= n {
                pre_ans += x;
                pre_ans %= MOD;
            }
        }

        ans.push(pre_ans);
    }

    println!("{}", ans.iter().join("\n"));
}

// fn main() {
//     input! {
//         t: usize,
//         nk: [(usize, usize); t]
//     }

//     let mut ans = vec![];

//     for &(n, k) in nk.iter() {
//         let mut m = 0_usize;
//         let mut x = n;
//         while x > 0 {
//             m += 1;
//             x /= 2;
//         }
//         if m < k {
//             ans.push(0_usize);
//             continue;
//         }
//         let mut dp = vec![vec![vec![0_usize; 2]; k]; m];
//         for i in 0..m {
//             let shift = m - i - 1;
//             dp[i][0][0] = 1;
//             dp[i][0][1] = dp[i.saturating_sub(1)][0][1];
//             if (dp[i][0][1] == 0) && ((n >> shift) & 1 != 0) {
//                 dp[i][0][1] = 1;
//             }
//             for j in 0..k - 1 {
//                 dp[i][j + 1][0] = dp[i.saturating_sub(1)][j + 1][0]
//                     + dp[i.saturating_sub(1)][j][0] * ((n >> shift) & 1);
//                 dp[i][j + 1][0] %= MOD;
//                 dp[i][j + 1][1] = dp[i.saturating_sub(1)][j + 1][1]
//                     + dp[i.saturating_sub(1)][j + 1][0] * ((n >> shift) & 1)
//                     + dp[i.saturating_sub(1)][j][1] * ((n >> shift) & 1);
//                 dp[i][j + 1][1] %= MOD;
//             }
//         }

//         ans.push(dp[m - 1][k - 1][0] + dp[m - 1][k - 1][1]);
//     }

//     println!("{}", ans.iter().join("\n"));
// }
