use proconio::input;
use proconio::marker::Chars;
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
        s: [Chars;8]
    }

    let mut board = [[true; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                for k in 0..8 {
                    board[i][k] = false;
                    board[k][j] = false;
                }
            }
        }
    }

    let mut ans = 0;

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
