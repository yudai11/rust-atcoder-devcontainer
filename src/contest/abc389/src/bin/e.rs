use num_integer::Roots;
use proconio::input;
// use proconio::marker::Chars;
use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::usize;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

// 貪欲

fn main() {
    input! {
        n: usize, m: usize,
        p: [usize; n]
    }

    // 二分探索
    let mut right = usize::MAX;
    let mut left: usize = 0;
    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }
        if can(mid, m, n, &p) {
            left = mid
        } else {
            right = mid
        }
    }

    let mut total: usize = 0;
    let mut cnt: usize = 0;
    for &pi in p.iter() {
        let num = (left / pi).sqrt() as usize;
        total += num * num * pi;
        cnt += num;
    }

    let ans = cnt + (m - total) / left;
    println!("{ans}");
}

fn can(x: usize, m: usize, n: usize, p: &Vec<usize>) -> bool {
    let mut total: usize = 0;
    for &pi in p.iter() {
        let num = (x / pi).sqrt() as usize;
        total += num * num * pi;
        if total > m {
            return false;
        }
    }
    return total <= m;
}
