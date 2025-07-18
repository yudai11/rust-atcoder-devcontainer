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
    }

    let mut ans = 1;

    for i in 1..n {
        let k = i * i * i;
        if k > n {
            break;
        }
        // kが回分数か判定

        if judge(k) {
            ans = k;
        }
    }

    println!("{ans}");
}

fn judge(k: usize) -> bool {
    let mut l = k;
    let mut num_vec = vec![];
    while l > 0 {
        num_vec.push(l % 10);
        l /= 10;
    }

    let m = num_vec.len();

    for i in 0..m {
        if num_vec[i] != num_vec[m - i - 1] {
            return false;
        }
    }

    return true;
}
