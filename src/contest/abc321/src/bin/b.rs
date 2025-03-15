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
        n: usize, x: usize,
        a: [usize; n-1]
    }

    let mut sum = 0_usize;
    let mut min_val = 100 * n;
    let mut max_val = 0_usize;

    for &ai in a.iter() {
        sum += ai;
        min_val = min_val.min(ai);
        max_val = max_val.max(ai);
    }

    if sum - min_val < x {
        // どんなスコアでも超えられない
        println!("-1");
    } else if sum - max_val >= x {
        // どんなスコアでも超えられる
        println!("0");
    } else {
        let ans = x + max_val + min_val - sum;
        println!("{}", ans);
    }
}
