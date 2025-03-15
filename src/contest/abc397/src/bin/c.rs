use proconio::{input, marker::Usize1};
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
        a: [Usize1; n]
    }

    let mut left_flag = vec![0; n];
    let mut right_flag = vec![0; n];

    let mut left_kind = 0_usize;
    let mut right_kind = 0_usize;

    left_flag[a[0]] += 1;
    left_kind += 1;

    for i in 1..n {
        let ai = a[i];
        if right_flag[ai] == 0 {
            right_kind += 1;
        }
        right_flag[ai] += 1;
    }

    let mut ans = left_kind + right_kind;

    for i in 1..n - 1 {
        let ai = a[i];
        if left_flag[ai] == 0 {
            left_kind += 1;
        }
        if right_flag[ai] == 1 {
            right_kind -= 1;
        }

        left_flag[ai] += 1;
        right_flag[ai] -= 1;

        ans = ans.max(left_kind + right_kind)
    }

    println!("{}", ans);
}
