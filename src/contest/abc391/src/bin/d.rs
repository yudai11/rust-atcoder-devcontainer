use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
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
        n: usize, w: usize,
        xy: [(Usize1,Usize1); n],
        q: usize,
    }

    let mut vanish_times = vec![1000_000_000_000; n];
    let mut _board = vec![vec![]; w];
    for i in 0..n {
        let (xi, yi) = xy[i];
        _board[xi].push((yi, i));
    }

    for i in 0..w {
        // 第1成分でsort
        _board[i].sort_by_key(|x| x.0);
        // _board[i].reverse();
    }

    let mut board = vec![VecDeque::new(); w];
    for i in 0..w {
        for j in 0.._board[i].len() {
            board[i].push(_board[i][j]);
        }
    }

    if board.iter().fold(true, |res, x| res && x.len() > 0) {
        let mut cur = 1_usize;

        if board.iter().fold(true, |res, x| res && x[0].0 == 0) {
            cur -= 1;
        }

        for i in 0..w {
            let x = board[i].pop_front().unwrap();
            vanish_times[x.1] = cur;
        }
        cur += 1;

        while board.iter().fold(true, |res, x| res && x.len() > 0) {
            for i in 0..w {
                let x = board[i].pop_front().unwrap();
                vanish_times[x.1] = cur;
            }
            cur += 2;
        }
    }

    for _i in 0..q {
        input! {
            t: usize, a: Usize1
        }
        if vanish_times[a] > t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
