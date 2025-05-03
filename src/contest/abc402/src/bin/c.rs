use itertools::Itertools;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
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
        n: usize, m: usize,
    }

    let mut dish_list = vec![vec![]; m];

    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k]
        }

        dish_list[i] = a.clone();
    }

    input! {
        b: [Usize1; n]
    }

    let mut when_overcome = vec![0_usize; n];
    for (i, &bi) in b.iter().enumerate() {
        when_overcome[bi] = i;
    }

    let mut ans = vec![0_usize; n];

    for i in 0..m {
        let mut when_eatable = 0_usize;
        for &ai in dish_list[i].iter() {
            when_eatable = when_eatable.max(when_overcome[ai]);
        }

        ans[when_eatable] += 1;
    }

    for i in 0..n - 1 {
        ans[i + 1] += ans[i];
    }

    println!("{}", ans.iter().join("\n"));
}
