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
        s: Chars,
    }

    let mut state = 0;
    let mut ans: Vec<usize> = vec![];
    let mut cnt = 0;

    for &x in s.iter() {
        if state == 0 {
            state += 1;
            continue;
        }
        if x == '|' {
            ans.push(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    for &x in ans.iter() {
        print!("{x} ");
    }
}
