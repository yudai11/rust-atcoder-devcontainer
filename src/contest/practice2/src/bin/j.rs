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
use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
use ac_library::{Max, Segtree}; // segtreeはisizeで使う

fn main() {
    input! {
        n: usize, q: usize,
        a: [isize;n]
    }
    let mut segtree = Segtree::<Max<isize>>::new(n);
    for i in 0..n {
        segtree.set(i, a[i]);
    }
    for _ in 0..q {
        input! {
            t: u8
        }
        match t {
            1 => {
                input! {
                    x: Usize1, v: isize
                }
                segtree.set(x, v);
            }
            2 => {
                input! {
                    l: Usize1, r: usize
                }
                println!("{}", segtree.prod(l..r));
            }
            _ => {
                input! {
                    x: Usize1, v: isize
                }
                println!("{}", segtree.max_right(x, |z| z < &v) + 1);
            }
        }
    }
}
