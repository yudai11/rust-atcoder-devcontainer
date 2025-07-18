use std::collections::HashMap;

use amplify::confinement::Collection;
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
        p: [Usize1; n],
        q: [Usize1; n]
    }

    let mut bib_to_person = HashMap::new();
    for i in 0..n {
        bib_to_person.push((q[i], i));
    }

    for i in 0..n {
        print!("{} ", q[p[bib_to_person[&i]]] + 1);
    }
}
