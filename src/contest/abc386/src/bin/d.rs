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
        n: usize, m: usize
    }

    // 各行について各色の右端を管理(defaultは0)
    let mut list_bh: Vec<usize> = vec![0; n];
    let mut list_wh: Vec<usize> = vec![0; n];

    // 各列について各色の下端を管理(defaultは0)
    let mut list_bv: Vec<usize> = vec![0; n];
    let mut list_wv: Vec<usize> = vec![0; n];

    for _ in 0..m {
        input! {
            x: Usize1, y: Usize1, c: char
        }
        if c == 'B' {
            list_bh[x] = list_bh[x].max(y);
            list_bv[x] = list_bv[x].max(x);
        }
        if c == 'C' {
            if c == 'B' {
                list_wh[x] = list_wh[x].max(y);
                list_wv[x] = list_wv[x].max(x);
            }
        }
    }
}
