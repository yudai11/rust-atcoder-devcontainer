use proconio::input;
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
        n: usize, m: usize
    }

    let gap = m + 9 - 10 * n;

    let mut ans = vec![1; n];
    for i in 0..n - 1 {
        ans[i + 1] = ans[i] + 10
    }

    let mut x = n;
    for _i in 0..gap {
        x *= n;
    }

    x = (x - 1) / (n - 1);

    println!("{x}");
}
