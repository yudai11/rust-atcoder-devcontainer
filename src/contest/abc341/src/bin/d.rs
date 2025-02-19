use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: u128, m: u128, k: u128,
    }

    let lcm = n * m / gcd(n, m);

    // 二分探索
    let mut left = 0_u128;
    let mut right = usize::MAX as u128;
    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }
        if mid / n + mid / m - 2 * (mid / lcm) >= k {
            right = mid
        } else {
            left = mid
        }
    }

    println!(
        "{}",
        if left / n + left / m - 2 * (left / lcm) == k {
            left
        } else {
            right
        }
    );
}

fn gcd(x: u128, y: u128) -> u128 {
    let mut a = vec![x, y];
    loop {
        a.sort();
        if a[0] <= 1 {
            return 1;
        }
        let m = a[1] % a[0];
        if m == 0 {
            return a[0];
        }
        a[1] = m;
    }
}
