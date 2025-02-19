use im_rc::HashMap;
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
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut next_map = HashMap::new();
    let mut start = 0_usize;
    for (i, &ai) in a.iter().enumerate() {
        if ai == -1 {
            start = i;
        } else {
            next_map.insert(ai as usize - 1, i);
        }
    }

    let mut ans = vec![];
    ans.push(start + 1);
    while next_map.contains_key(&start) {
        start = next_map[&start];
        ans.push(start + 1);
    }

    println!("{}", ans.iter().join(" "));
}
