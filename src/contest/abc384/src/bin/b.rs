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
        n: usize, mut r: isize,
    }

    for _i in 0..n {
        input! {
            d: u8, a: isize
        }

        if d == 1 {
            if 1600 <= r && r <= 2799 {
                r += a;
            }
        }

        if d == 2 {
            if 1200 <= r && r <= 2399 {
                r += a;
            }
        }
    }

    println!("{r}");
}
