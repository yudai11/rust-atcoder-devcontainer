use itertools::Itertools;
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
        n: usize, m: usize,
        mut a: [usize; m]
    }

    a.sort();

    let mut i = 0_usize;
    let mut cnt = 0_usize;
    let mut ans = vec![];
    for j in 1..=n {
        if i < m && j == a[i] {
            i += 1
        } else {
            cnt += 1;
            ans.push(j);
        }
    }

    println!("{}", cnt);
    println!("{}", ans.iter().join(" "))
}
