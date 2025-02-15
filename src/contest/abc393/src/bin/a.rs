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
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;

fn main() {
    input! {
        s1: String, s2: String
    }

    let d1 = s1 == "sick";
    let d2 = s2 == "sick";

    if d1 && d2 {
        println!("1");
    } else if d1 && !d2 {
        println!("2");
    } else if !d1 && d2 {
        println!("3");
    } else {
        println!("4");
    }
}
