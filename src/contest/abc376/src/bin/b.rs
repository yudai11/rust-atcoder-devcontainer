use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, q: usize,
        ht: [(char,Usize1);q]
    }

    let mut right_hand: usize = 1;
    let mut left_hand: usize = 0;
    let mut ans = 0;
    for i in 0..q {
        let (h, t) = ht[i];
        let mut dist = n;
        if h == 'L' {
            if left_hand == t {
                dist = 0;
            }
            if (left_hand < t && t < right_hand) || (right_hand < left_hand && left_hand < t) {
                dist = dist.min(t - left_hand);
            }
            if t < right_hand && right_hand < left_hand {
                dist = dist.min(n + t - left_hand);
            }
            if (right_hand < t && t < left_hand) || (t < left_hand && left_hand < right_hand) {
                dist = dist.min(left_hand - t);
            }
            if left_hand < right_hand && right_hand < t {
                dist = dist.min(n + left_hand - t);
            }

            left_hand = t;
        } else {
            if right_hand == t {
                dist = 0;
            }
            if (left_hand < t && t < right_hand) || (t < right_hand && right_hand < left_hand) {
                dist = dist.min(right_hand - t);
            }
            if right_hand < left_hand && left_hand < t {
                dist = dist.min(n + right_hand - t);
            }
            if (right_hand < t && t < left_hand) || (left_hand < right_hand && right_hand < t) {
                dist = dist.min(t - right_hand);
            }
            if t < left_hand && left_hand < right_hand {
                dist = dist.min(n + t - right_hand);
            }

            right_hand = t;
        }

        ans += dist;
    }

    println!("{ans}");
}
