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

fn main() {
    input! {
        n: usize, h: usize, m: usize,
        ab: [(usize,usize); n]
    }

    // 消費体力がiのときの最小消費魔力（オンラインdp）
    let mut dp: Vec<usize> = vec![1000_000_000_000_usize; h + 1];
    dp[0] = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        let mut ndp = vec![1000_000_000_000_usize; h + 1];
        let mut can_defeat = false;

        for j in 0..=h {
            if dp[j] + b <= m {
                can_defeat = true;
                ndp[j] = ndp[j].min(dp[j] + b);
            }
            if j + a <= h {
                if dp[j] <= m {
                    can_defeat = true;
                }
                ndp[j + a] = ndp[j + a].min(dp[j]);
            }
        }

        if can_defeat {
            dp = ndp;
        } else {
            println!("{}", i);
            return;
        }
    }

    println!("{}", n);
}
