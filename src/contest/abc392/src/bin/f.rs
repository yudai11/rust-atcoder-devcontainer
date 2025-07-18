use proconio::input;
// use proconio::marker::Chars;
use itertools::Itertools;
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
use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut seg_tree = Segtree::<Additive<_>>::from(vec![1; n]);

    let mut ans = vec![0; n];
    for (i, &p) in p.iter().enumerate().rev() {
        let j = seg_tree.max_right(0, |&m| m < p);
        ans[j] = i + 1;
        seg_tree.set(j, 0);
    }

    println!("{}", ans.iter().join(" "));
}
