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
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m]
    }

    let mut c = vec![a[0]; n];
    let mut cur_min = a[0];
    for i in 1..n {
        cur_min = cur_min.min(a[i]);
        c[i] = cur_min;
    }

    for &bi in b.iter() {
        let mut left = 0;
        let mut right = n + 1;
        let mut mid = n;
        loop {
            mid = left + (right - left) / 2;
            if mid == left {
                break;
            }
            if c[mid - 1] > bi {
                left = mid;
            } else {
                right = mid;
            }
        }

        if mid >= n {
            println!("-1");
        } else if mid > 0 {
            println!("{}", mid + 1);
        } else {
            println!("1");
        }
    }
}
