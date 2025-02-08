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
        n: usize, m: usize,
        s: [Chars; n],
        t: [Chars; m]
    }

    for i in 0..n {
        for j in 0..n {
            if i + m > n || j + m > n {
                continue;
            }
            if check(i, j, &s, &t, m) {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}

fn check(x: usize, y: usize, s: &Vec<Vec<char>>, t: &Vec<Vec<char>>, m: usize) -> bool {
    for i in 0..m {
        for j in 0..m {
            if s[x + i][y + j] != t[i][j] {
                return false;
            }
        }
    }

    return true;
}
