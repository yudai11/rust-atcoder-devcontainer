use indexing::container_traits::Pushable;
use itertools::Itertools;
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
        q: usize
    }

    // ..i番目までjがあることをさす
    let mut row = vec![];
    // row.push((0, 0));
    let mut row_end = 0_usize;
    let mut cur_loc = 0_usize;
    let mut cur_block = 0_usize;

    let mut ans = vec![];

    for _i in 0..q {
        input! {
            t: usize
        }

        match t {
            1 => {
                input! {
                    c: usize, x: usize
                }
                row_end += c;
                row.push((row_end, x));
            }
            2 => {
                input! {
                    k: usize
                }
                let mut res = 0_usize;
                let end = cur_loc + k;
                while row[cur_block].0 < end {
                    res += row[cur_block].1 * (row[cur_block].0 - cur_loc);
                    cur_loc = row[cur_block].0;
                    cur_block += 1;
                }
                res += row[cur_block].1 * (end - cur_loc);
                cur_loc = end;
                ans.push(res);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}
