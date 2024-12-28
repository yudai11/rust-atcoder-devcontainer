use proconio::{input, marker::Usize1};
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
        p: [Usize1;n],
        q: usize,
    }

    let mut loc_list: Vec<usize> = vec![0; n];

    for (i, &pi) in p.iter().enumerate() {
        loc_list[pi] = i;
    }

    for _ in 0..q {
        input! {
            a: Usize1, b: Usize1
        }

        if loc_list[a] < loc_list[b] {
            println!("{}", a + 1);
        } else {
            println!("{}", b + 1);
        }
    }
}
