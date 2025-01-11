use std::collections::HashSet;

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
    // let mut seen = HashSet::new();

    let mut left: usize = 0;
    let mut right: usize = n / 2 + 1;
    let mut mid: usize = n;
    loop {
        mid = left + (right - left) / 2;
        if mid == left || mid >= n {
            break;
        }
        if judge(&a[0..mid], &a[mid..n]) {
            left = mid;
        } else {
            right = mid;
        }
    }

    // if mid < n {
    //     if a[mid] >= 2 * up {
    //         ans += n - mid;
    //     } else {
    //         ans += n - mid - 1;
    //     }
    // }

    println!("{mid}");
}

fn judge(ups: &[usize], lows: &[usize]) -> bool {
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < ups.len() && j < lows.len() {
        if ups[i] * 2 <= lows[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    if i >= ups.len() {
        return true;
    } else {
        return false;
    }
}
