use itertools::Itertools;
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
        n: usize
    }

    let mut dices = vec![vec![0.0 as f64; 100001]; n];
    for i in 0..n {
        input! {
            ki: usize,
            ai:[usize; ki]
        }

        for aii in ai {
            dices[i][aii] += 1.0 / ki as f64;
        }
    }

    let mut ans = 0_f64;

    for com in (0..n).combinations(2) {
        ans = ans.max(f(&dices[com[0]], &dices[com[1]]))
    }

    println!("{}", ans);
}

fn f(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let mut res = 0_f64;
    for i in 0..=100000 {
        res += x[i] * y[i];
    }

    return res;
}
