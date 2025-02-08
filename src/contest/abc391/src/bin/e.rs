use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        _a: Chars
    }

    let mut a = vec![];
    let mut m = _a.len();

    for i in 0..m {
        a.push(_a[i] as u8 - '0' as u8);
    }

    let mut cost_change = vec![vec![]; n - 1];
    for i in 0..n {
        m /= 3;
        for i in 0..m {
            let x = a[3 * i] + a[3 * i + 1] + a[3 * i + 2];
        }
    }
}
