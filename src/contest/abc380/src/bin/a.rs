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
        mut n: usize,
    }

    let mut cnt = [0; 10];
    while n > 0 {
        cnt[n % 10] += 1;
        n /= 10;
    }

    let mut feasi = true;

    for i in 1..=3 {
        if cnt[i] != i {
            feasi = false;
        }
    }

    if feasi {
        println!("Yes");
    } else {
        println!("No");
    }
}
