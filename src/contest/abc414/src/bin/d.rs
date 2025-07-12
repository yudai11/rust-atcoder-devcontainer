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
        n: usize, m: usize,
        mut x: [usize; n]
    }

    x.sort();

    let mut locs = vec![];
    let mut gaps = vec![];
    for &xi in x.iter() {
        if let Some(y) = locs.pop() {
            if y < xi {
                locs.push(y);
                gaps.push(xi - y);
            }
        }
        locs.push(xi);
    }

    let l = locs.len();

    gaps.sort();
    gaps.reverse();

    let mut ans = locs[l - 1] - locs[0];

    for i in 0..(l - 1).min(m - 1) {
        ans -= gaps[i];
    }

    println!("{}", ans);
}
