use itertools::Itertools;
use proconio::{input, marker::Chars};
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
        a: [usize; m],
        s: [Chars; n]
    }

    let mut max_val = 0_usize;
    for (i, si) in s.iter().enumerate() {
        let mut max_candi = i;
        for j in 0..m {
            match si[j] {
                'o' => {
                    max_candi += a[j];
                }
                'x' => {}
                _ => unreachable!(),
            }
        }
        max_val = max_val.max(max_candi);
    }

    let mut ans = vec![];
    for (i, si) in s.iter().enumerate() {
        let mut score = i;
        let mut can_chosen = vec![];
        for j in 0..m {
            match si[j] {
                'o' => {
                    score += a[j];
                }
                'x' => {
                    can_chosen.push(a[j]);
                }
                _ => unreachable!(),
            }
        }
        can_chosen.sort();
        let mut cnt = 0_usize;

        while let Some(plus) = can_chosen.pop() {
            if score >= max_val {
                break;
            }
            cnt += 1;
            score += plus;
        }

        ans.push(cnt);
    }

    println!("{}", ans.iter().join("\n"));
}
