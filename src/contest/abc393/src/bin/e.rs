// use itertools::Itertools;
use proconio::input;
// use std::{cmp::Reverse, collections::HashSet};
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
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut m = a.iter().fold(0_usize, |res, &x| res.max(x));

    let mut occurences = vec![0_usize; m + 1];
    for &ai in a.iter() {
        occurences[ai] += 1;
    }

    let mut t = vec![0_usize; m + 1];
    for i in 1..=m {
        for j in (i..=m).step_by(i) {
            t[i] += occurences[j];
        }
    }

    let mut u = vec![0_usize; m + 1];
    for i in 1..=m {
        if t[i] < k {
            continue;
        }
        for j in (i..=m).step_by(i) {
            u[j] = u[j].max(i);
        }
    }

    for i in 0..n {
        println!("{}", u[a[i]]);
    }
}
