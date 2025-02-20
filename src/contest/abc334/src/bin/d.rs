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
        n: usize, q: usize,
        mut r: [usize; n],
        querys: [usize; q]
    }

    r.sort();
    let mut cum_sum = vec![0_usize; n];
    cum_sum[0] = r[0];
    for i in 1..n {
        cum_sum[i] = cum_sum[i - 1] + r[i];
    }

    for x in querys {
        let ind = cum_sum.lower_bound(&(x + 1));
        println!("{}", ind);
    }
}
