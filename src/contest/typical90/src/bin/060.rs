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
        n: usize,
        mut a: [usize; n]
    }

    // i 個の連続する部分増加列のうち末尾が最小のものの末尾 / iについて単調増加する
    let mut dp_inc = vec![1_usize; n];
    // 長さiの単調増加列の中で末尾が最小なものを記録
    let mut least_ends = vec![1000_000_000_usize; n + 1];
    least_ends[0] = 0;
    for i in 0..n {
        let ind = least_ends.lower_bound(&a[i]);
        dp_inc[i] = ind;
        least_ends[dp_inc[i]] = least_ends[dp_inc[i]].min(a[i]);
    }

    a.reverse();
    // i 個の連続する部分増加列のうち末尾が最小のものの末尾 / iについて単調増加する
    let mut dp_dec = vec![1_usize; n];
    // 長さiの単調増加列の中で末尾が最小なものを記録
    let mut least_ends = vec![1000_000_000_usize; n + 1];
    least_ends[0] = 0;
    for i in 0..n {
        let ind = least_ends.lower_bound(&a[i]);
        dp_dec[i] = ind;
        least_ends[dp_dec[i]] = least_ends[dp_dec[i]].min(a[i]);
    }
    dp_dec.reverse();

    let mut ans = 1_usize;
    for i in 0..n {
        ans = ans.max(dp_inc[i] + dp_dec[i] - 1);
    }

    println!("{}", ans);
}
