use std::collections::HashSet;

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

    let mut feasi = true;
    let mut state = 'a';
    let n = s.len();
    let mut seen = HashSet::new();

    if n % 2 != 0 {
        println!("No");
        return;
    }

    for i in 0..n {
        if i % 2 == 0 {
            state = s[i];
            if seen.contains(&state) {
                feasi = false;
                break;
            }
            seen.insert(state);
        } else {
            if state != s[i] {
                feasi = false;
                break;
            }
        }
    }

    if feasi {
        println!("Yes");
    } else {
        println!("No");
    }
}
