use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize, m: usize, a: usize, b: usize,
        lr: [(Usize1,Usize1);m]
    }

    let mut index: usize = 0;
    let mut loc_set: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    loc_set.push(Reverse(0));

    while let Some(Reverse(min_value)) = loc_set.pop() {}
}
