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
        xyz: [(Usize1,Usize1,usize); m]
    }

    // Xj = i となる jを格納する
    let mut x_emerge = vec![vec![]; n];
    let mut y_emerge = vec![vec![]; n];

    for (i, &(x, y, _z)) in xyz.iter().enumerate() {
        x_emerge[x].push(i);
        y_emerge[y].push(i);
    }

    let mut feasi = true;

    for i in 0..n {
        if x_emerge[i].len() > 1 {
            for j in x_emerge[i] {}
        }
    }
}
