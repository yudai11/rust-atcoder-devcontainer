use proconio::input;
use proconio::marker::Chars;
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
        h: usize, w: usize,
        s: [Chars;h]
    }

    let mut t: usize = h;
    let mut b: usize = 0;
    let mut l: usize = w;
    let mut r: usize = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                t = t.min(i);
                b = b.max(i);
                l = l.min(j);
                r = r.max(j);
            }
            if s[i][j] == '?' {}
        }
    }

    for i in t..=b {
        for j in l..=r {
            if s[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
