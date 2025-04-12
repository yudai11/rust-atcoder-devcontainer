use ac_library::Dsu;
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
        edges: [(Usize1,Usize1); m],
    }

    let mut is_connect_1_k = Dsu::new(k);
    let mut uf = Dsu::new(n);

    for &(u, v) in edges.iter() {
        uf.merge(u, v);
        if u < k && v < k {
            is_connect_1_k.merge(u, v);
        }
    }

    let leader_set = is_connect_1_k.groups();
    if leader_set.len() != 1 {
        println!("-1");
        return;
    } else {
        let x = uf.groups();
        let l = x.len();
        for i in 0..l {
            if !uf.same(0, x[i][0]) {
                continue;
            } else {
                println!("{}", x[i].len() - k);
                return;
            }
        }
    }
}
