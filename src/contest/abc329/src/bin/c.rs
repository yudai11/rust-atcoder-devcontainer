use proconio::{input, marker::Chars};
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
        n: usize,
        s: Chars
    }

    let mut max_conti = vec![0_usize; 26];
    let mut i = 0_usize;
    while i < n {
        let ind = s[i] as usize - 'a' as usize;
        let mut conti = 1_usize;
        while i < n - 1 && s[i + 1] == s[i] {
            conti += 1;
            i += 1;
        }
        max_conti[ind] = max_conti[ind].max(conti);
        i += 1;
    }

    let ans = max_conti.iter().sum::<usize>();
    println!("{}", ans);
}
