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
        n: usize, m: usize,
        lr: [(Usize1,Usize1); m]
    }

    let mut defense = vec![0_isize; n + 1];
    for &(li, ri) in lr.iter() {
        defense[li] += 1;
        defense[ri + 1] -= 1;
    }

    for i in 0..n {
        defense[i + 1] += defense[i];
    }

    let mut ans = m as isize;
    for i in 0..n {
        ans = ans.min(defense[i]);
    }

    println!("{}", ans);
}
