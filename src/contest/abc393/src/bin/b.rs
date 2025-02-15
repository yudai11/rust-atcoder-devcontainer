use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
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
        s: Chars,
    }

    let mut loc_a = vec![];
    let mut loc_b = vec![];
    let mut loc_c = HashSet::new();
    for (i, &si) in s.iter().enumerate() {
        if si == 'A' {
            loc_a.push(i);
        }
        if si == 'B' {
            loc_b.push(i);
        }
        if si == 'C' {
            loc_c.insert(i);
        }
    }

    let mut ans = 0_usize;

    for &ai in loc_a.iter() {
        for &bi in loc_b.iter() {
            if ai < bi && loc_c.contains(&(2 * bi - ai)) {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
