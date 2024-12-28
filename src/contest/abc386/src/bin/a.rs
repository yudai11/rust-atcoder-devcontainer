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
        a: [usize;4]
    }

    let mut num_c: [usize; 14] = [0; 14];
    for i in 0..4 {
        num_c[a[i]] += 1;
    }

    let mut num_t = 0;
    let mut num_d = 0;
    for &x in num_c.iter() {
        if x == 3 {
            num_t += 1;
        }
        if x == 2 {
            num_d += 1;
        }
    }

    if num_t == 1 || num_d == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
