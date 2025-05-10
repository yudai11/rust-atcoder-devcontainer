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
        n: usize,
        s: [Chars; n],
        t: [Chars; n]
    }

    let mut ans = n * n;

    let mut pre_ans = 0_usize;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                pre_ans += 1;
            }
        }
    }
    ans = ans.min(pre_ans);

    pre_ans = 1_usize;
    for i in 0..n {
        for j in 0..n {
            if s[n - 1 - j][i] != t[i][j] {
                pre_ans += 1;
            }
        }
    }
    ans = ans.min(pre_ans);

    pre_ans = 2_usize;
    for i in 0..n {
        for j in 0..n {
            if s[n - 1 - i][n - 1 - j] != t[i][j] {
                pre_ans += 1;
            }
        }
    }
    ans = ans.min(pre_ans);

    pre_ans = 3_usize;
    for i in 0..n {
        for j in 0..n {
            if s[j][n - 1 - i] != t[i][j] {
                pre_ans += 1;
            }
        }
    }
    ans = ans.min(pre_ans);

    println!("{}", ans);
}
