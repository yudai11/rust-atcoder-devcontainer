use proconio::{input, marker::Chars};
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
        s: Chars
    }

    let mut cnt_alph = [0; 26];

    for i in 0..s.len() {
        cnt_alph[s[i] as usize - 'a' as usize] += 1;
    }

    let mut x: char = 'a';
    for i in 0..26 {
        if cnt_alph[i] == 1 {
            x = (i as u8 + 'a' as u8) as char;
        }
    }

    for i in 0..s.len() {
        if s[i] == x {
            println!("{}", i + 1);
        }
    }
}
