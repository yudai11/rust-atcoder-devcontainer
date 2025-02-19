use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
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
        a: [Chars; n]
    }

    let mut ans = vec![vec![0; n]; n];
    ans[0][0] = a[1][0] as usize - '0' as usize;
    for i in 1..n {
        ans[0][i] = a[0][i - 1] as usize - '0' as usize;
        ans[i][n - 1] = a[i - 1][n - 1] as usize - '0' as usize;
        ans[n - 1][n - 1 - i] = a[n - 1][n - i] as usize - '0' as usize;
        ans[n - 1 - i][0] = a[n - i][0] as usize - '0' as usize;
    }
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            ans[i][j] = a[i][j] as usize - '0' as usize;
        }
    }

    for i in 0..n {
        println!("{}", ans[i].iter().join(""));
    }
}
