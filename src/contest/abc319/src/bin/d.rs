use itertools::Itertools;
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
use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize, m: usize,
        l: [usize; n]
    }

    let mut cum_sum = vec![0_usize; n];
    cum_sum[0] = l[0];
    for i in 1..n {
        cum_sum[i] = cum_sum[i - 1] + 1 + l[i];
    }
    // println!("{}", cum_sum.iter().join(" "));

    // 二分探索
    let mut right = 1000_000_000_000_001;
    let mut left: usize = 0;
    // can(25, &cum_sum, &l, m, n);
    // can(26, &cum_sum, &l, m, n);

    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }
        if can(mid, &cum_sum, &l, m, n) {
            right = mid
        } else {
            left = mid
        }
    }

    println!("{}", right);
}

fn can(mid: usize, cum_sum: &Vec<usize>, l: &Vec<usize>, m: usize, n: usize) -> bool {
    let mut num_low = 1_usize;
    let mut cur_sum = 0_usize;

    for i in 0..n {
        if l[i] > mid {
            return false;
        }
        cur_sum += l[i] + 1;
        if cur_sum > mid + 1 {
            cur_sum = l[i] + 1;
            num_low += 1;
            if num_low > m {
                return false;
            }
        }
    }

    return num_low <= m;
}
