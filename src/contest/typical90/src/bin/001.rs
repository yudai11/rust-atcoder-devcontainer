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
        n: usize, l: usize,
        k: usize,
        mut a: [usize;n]
    }

    let mut left = 1;
    let mut right = l / (k + 1) + 1;

    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }

        if can(mid, &a, l, n, k) {
            left = mid
        } else {
            right = mid
        }
    }

    println!("{}", left);
}

fn can(mid: usize, a: &Vec<usize>, l: usize, n: usize, k: usize) -> bool {
    let mut i = 0_usize;
    let mut cur_loc = 0_usize;
    let mut num_piece = 0_usize;
    while i < n {
        if a[i] - cur_loc >= mid {
            num_piece += 1;
            cur_loc = a[i];
        }
        i += 1;
    }
    if l - cur_loc >= mid {
        num_piece += 1;
    }

    if num_piece > k {
        return true;
    } else {
        return false;
    }
}
