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
        n: usize,
        a: [usize;n]
    }

    let mut ans: usize = 0;

    for i in 0..n {
        let up: usize = a[i];

        let mut left: usize = i;
        let mut right: usize = n + 1;
        let mut mid: usize = n;
        loop {
            mid = left + (right - left) / 2;
            if mid == left || mid >= n {
                break;
            }
            if a[mid] < 2 * up {
                left = mid;
            } else {
                right = mid;
            }
        }

        if mid < n {
            if a[mid] >= 2 * up {
                ans += n - mid;
            } else {
                ans += n - mid - 1;
            }
        }
    }

    println!("{ans}");
}
