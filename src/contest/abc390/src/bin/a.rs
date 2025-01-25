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
        a: [Usize1;5]
    }

    let mut inco = 0;
    let mut b = vec![];
    for i in 0..5 {
        if i != a[i] {
            inco += 1;
            b.push(i);
        }
    }

    if inco != 2 {
        println!("No");
    } else if b[1] - b[0] != 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
