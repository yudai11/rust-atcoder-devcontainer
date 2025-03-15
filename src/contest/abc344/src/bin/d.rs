use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        t: Chars,
        n: usize,
    }

    let l = t.len();
    // tのi文字目まで完成させるのに必要なコスト
    let mut dp = vec![1000_000_000_usize; l + 1];
    dp[0] = 0;

    for _i in 0..n {
        input! {
            a: usize,
            s: [Chars; a]
        }

        let temp = dp.clone();

        for si in s.iter() {
            if l < si.len() {
                continue;
            }
            for j in 0..=l - si.len() {
                for k in 0..si.len() {
                    if t[j + k] != si[k] {
                        break;
                    }
                    if k == si.len() - 1 {
                        dp[j + k + 1] = dp[j + k + 1].min(temp[j] + 1);
                    }
                }
            }
        }
    }

    if dp[l] > 1000_000_00 {
        println!("-1");
    } else {
        println!("{}", dp[l]);
    }
}
