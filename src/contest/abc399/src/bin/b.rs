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

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut x = vec![];
    for (i, &pi) in p.iter().enumerate() {
        x.push((pi, i));
    }

    x.sort_by(|x, y| y.0.cmp(&x.0));

    let mut rank = vec![0_usize; n];
    let mut i = 0_usize;

    while i < n {
        let max_val = x[i].0;
        rank[x[i].1] = i + 1;
        let mut slide = 1_usize;
        while i + slide < n && x[i + slide].0 == max_val {
            rank[x[i + slide].1] = i + 1;
            slide += 1;
        }
        i += slide;
    }

    println!("{}", rank.iter().join("\n"));
}
