use proconio::{input, marker::Usize1};
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
        xx: [Usize1; m],
        aa: [usize; m],
    }

    let mut xa = vec![];
    for i in 0..m {
        xa.push((xx[i], aa[i]));
    }

    xa.sort_by_key(|&(x, _)| x);

    let mut cum_sum: usize = 0;
    for &(xi, ai) in xa.iter().rev() {
        cum_sum += ai;
        if cum_sum > n {
            println!("-1");
            return;
        }
        if cum_sum > n - xi {
            println!("-1");
            return;
        }
    }

    if cum_sum != n {
        println!("-1");
        return;
    }

    // 並べ替えはできる
    let mut ans: u128 = (n * (n - 1) / 2) as u128;

    for &(xi, ai) in xa.iter() {
        ans -= (xi * ai) as u128;
    }

    println!("{ans}");
}
