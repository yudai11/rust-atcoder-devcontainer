use std::collections::HashSet;

use itertools::Itertools;
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

// 破壊，燃やす，埋める，回収

fn main() {
    input! {
        h: usize, w: usize, n: usize,
        xy: [(Usize1,Usize1); n],
        q: usize,
        qq: [(u8,Usize1); q]
    }

    let mut x_set = vec![HashSet::new(); h];
    let mut y_set = vec![HashSet::new(); w];

    for &(x, y) in xy.iter() {
        x_set[x].insert(y);
        y_set[y].insert(x);
    }

    let mut ans = vec![];

    for &(t, z) in qq.iter() {
        match t {
            1 => {
                ans.push(x_set[z].len());
                for &u in x_set[z].iter() {
                    y_set[u].remove(&z);
                }
                x_set[z].clear();
            }
            2 => {
                ans.push(y_set[z].len());
                for &u in y_set[z].iter() {
                    x_set[u].remove(&z);
                }
                y_set[z].clear();
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}
