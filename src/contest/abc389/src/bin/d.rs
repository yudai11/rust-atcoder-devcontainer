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
        r: usize
    }

    // 十字方向のタイル数
    let cross = 4 * r - 3;
    let mut x: usize = 1;
    let mut y: usize = r - 1;
    let mut tr_area: usize = 0;
    while x < r && y > 0 {
        let x2 = x as f64 + 0.5;
        let y2 = y as f64 + 0.5;
        if x2 * x2 + y2 * y2 <= r as f64 * r as f64 {
            tr_area += y;
            x += 1;
        } else {
            y -= 1;
        }
    }

    let ans = cross + 4 * tr_area;
    println!("{ans}");
}
