use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut b: Vec<usize> = vec![0; n];
    let mut gaps = BinaryHeap::new();

    // let mut thteshold: usize = 0;
    let mut plus: usize = 0;

    for i in 0..n {
        // 付与できないものをすべて消す．
        while let Some(Reverse(min_value)) = gaps.pop() {
            if min_value >= i {
                gaps.push(Reverse(min_value));
                break;
            } else {
                if plus > 0 {
                    plus -= 1;
                }
            }
        }

        // plus = gaps.len();

        let bi: usize = if a[i] + plus >= n - i - 1 {
            a[i] + plus + i + 1 - n
        } else {
            0
        };

        gaps.push(Reverse(a[i] + plus + i));
        plus += 1;
        b[i] = bi;
    }

    for i in 0..n {
        print!("{} ", b[i]);
    }
}
