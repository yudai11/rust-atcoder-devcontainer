use std::collections::VecDeque;

use proconio::input;
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
        q: usize
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut highest: usize = 0;
    let mut tree_0: usize = 0;

    for _ in 0..q {
        input! {
            v: usize
        }
        match v {
            1 => {
                tree_0 += 1;
            }
            2 => {
                input! {
                    t: usize,
                }
                queue.push_back((t, tree_0));
                highest += t;
                tree_0 = 0;
            }
            3 => {
                input! {
                    h: usize,
                }
                if highest < h {
                    println!("0");
                    continue;
                }
                let mut tree_height: usize = highest;
                let mut tree_num: usize = 0;
                loop {
                    if queue.len() == 0 {
                        println!("{tree_num}");
                        highest = 0;
                        break;
                    }
                    let (x, y) = queue.pop_front().unwrap();
                    tree_height -= x;
                    tree_num += y;
                    if tree_height < h {
                        // queue.push_front((x, y));
                        println!("{tree_num}");
                        highest = tree_height;
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
