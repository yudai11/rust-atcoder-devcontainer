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
        n: usize,
        q: usize
    }

    let mut ans = 0_isize;
    let mut holes = vec![1_usize; n];
    let mut locs = vec![0_usize; n];
    for i in 0..n {
        locs[i] = i;
    }

    for _i in 0..q {
        input! {
            x: u8,
        }
        match x {
            1 => {
                input! {
                    p: Usize1, h: Usize1,
                }
                if holes[locs[p]] == 2 {
                    ans -= 1;
                }
                holes[locs[p]] -= 1;
                locs[p] = h;
                holes[h] += 1;
                if holes[h] == 2 {
                    ans += 1
                }
            }
            2 => {
                println!("{ans}");
            }
            _ => {}
        }
    }
}
