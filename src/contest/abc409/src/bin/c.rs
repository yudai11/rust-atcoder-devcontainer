use proconio::input;
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
        n: usize, l: usize,
        d: [usize; n-1]
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut ans = 0_usize;

    let mut is_exist = vec![0_usize; l];
    let mut pos = vec![0_usize; 2 * n];
    is_exist[0] += 1;
    for i in 1..n {
        pos[i] = pos[i - 1] + d[i - 1];
        is_exist[pos[i] % l] += 1;
    }
    // pos[n] = l;
    // is_exist[l] = true;
    // for i in 1..n {
    //     pos[n + i] = pos[n + i - 1] + d[i - 1];
    //     is_exist[pos[n + i] % l] = true;
    // }

    let gap = l / 3;

    for i in 0..n {
        ans += is_exist[(pos[i] + gap) % l] * is_exist[(pos[i] + 2 * gap) % l];
    }

    println!("{}", ans / 3);
}
