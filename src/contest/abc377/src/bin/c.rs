use std::collections::HashSet;

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

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1,Usize1);m]
    }

    let mut ans = n * n;

    let mut board = HashSet::new();

    for &(a, b) in ab.iter() {
        if !board.contains(&(a * n + b)) {
            ans -= 1;
            board.insert(a * n + b);
        }

        if a + 2 < n && b + 1 < n && !board.contains(&((a + 2) * n + b + 1)) {
            ans -= 1;
            board.insert((a + 2) * n + b + 1);
        }

        if a + 1 < n && b + 2 < n && !board.contains(&((a + 1) * n + (b + 2))) {
            ans -= 1;
            board.insert((a + 1) * n + (b + 2));
        }

        if a >= 1 && b + 2 < n && !board.contains(&((a - 1) * n + (b + 2))) {
            ans -= 1;
            board.insert((a - 1) * n + (b + 2));
        }

        if a >= 2 && b + 1 < n && !board.contains(&((a - 2) * n + (b + 1))) {
            ans -= 1;
            board.insert((a - 2) * n + (b + 1));
        }

        if a >= 2 && b >= 1 && !board.contains(&((a - 2) * n + (b - 1))) {
            ans -= 1;
            board.insert((a - 2) * n + (b - 1));
        }

        if a >= 1 && b >= 2 && !board.contains(&((a - 1) * n + (b - 2))) {
            ans -= 1;
            board.insert((a - 1) * n + (b - 2));
        }

        if a + 1 < n && b >= 2 && !board.contains(&((a + 1) * n + (b - 2))) {
            ans -= 1;
            board.insert((a + 1) * n + (b - 2));
        }

        if a + 2 < n && b >= 1 && !board.contains(&((a + 2) * n + (b - 1))) {
            ans -= 1;
            board.insert((a + 2) * n + (b - 1));
        }
    }

    println!("{ans}");
}
