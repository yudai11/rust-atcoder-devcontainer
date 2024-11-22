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
        n: usize,
        s: Chars
    }

    if n % 2 == 0 {
        println!("No");
        return;
    }

    let m = (n - 1) / 2;
    let mut feasi = true;
    for i in 0..m {
        if s[i] != '1' {
            feasi = false;
            break;
        }
    }
    if s[m] != '/' {
        feasi = false;
    }
    for i in m + 1..n {
        if s[i] != '2' {
            feasi = false;
            break;
        }
    }

    if feasi {
        println!("Yes");
    } else {
        println!("No");
    }
}
