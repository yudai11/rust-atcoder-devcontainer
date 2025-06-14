use proconio::input;
use std::collections::HashSet;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
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
        h: usize, w: usize,
        a: [[usize; w]; h]
    }

    let mut ans = 0_usize;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(0_usize);
    while let Some(bit) = queue.pop_front() {
        seen.insert(bit);
        let mut pre_ans = 0_usize;
        for i in 0..h * w {
            if (bit >> i) & 1 == 0 {
                pre_ans ^= a[i / w][i % w];
            }
        }
        ans = ans.max(pre_ans);

        for i in 0..h * w {
            if i % w < w - 1 && ((bit >> i) & 1 == 0) && ((bit >> (i + 1)) & 1 == 0) {
                if !seen.contains(&(bit | (1 << i) | (1 << (i + 1)))) {
                    queue.push_back(bit | (1 << i) | (1 << (i + 1)));
                    seen.insert(bit | (1 << i) | (1 << (i + 1)));
                }
            }
            if i / w < h - 1 && ((bit >> i) & 1 == 0) && ((bit >> (i + w)) & 1 == 0) {
                if !seen.contains(&(bit | (1 << i) | (1 << (i + w)))) {
                    queue.push_back(bit | (1 << i) | (1 << (i + w)));
                    seen.insert(bit | (1 << i) | (1 << (i + w)));
                }
            }
        }
    }

    println!("{}", ans);
}
