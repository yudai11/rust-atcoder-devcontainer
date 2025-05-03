use amplify::confinement::Collection;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
use std::collections::HashSet;
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
        q: usize,
    }

    let mut x = HashSet::new();

    let mut y = vec![];

    for _i in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! {
                s: String
            }
            x.insert(s);
        } else {
            input! {
                s: Chars
            }
            y.push(s)
        }
    }

    let mut n = y.len();

    for i in 0..n {
        let m = y[i].len();
        let mut feasi = true;
        for j in 0..m {
            y[i]
        }
    }
}
