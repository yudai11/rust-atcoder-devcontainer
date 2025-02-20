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

// 単調性に注目する．(xの増加に対する黒の範囲の単調減少)
fn main() {
    input! {
        n: usize, m: usize,
        mut xyc: [(Usize1,Usize1,char);m]
    }

    // 第0要素->第1要素の順に昇順ソート(辞書順)
    xyc.sort_by(|&x, &y| {
        if x.0 != y.0 {
            x.0.cmp(&y.0)
        } else if x.1 != y.1 {
            x.1.cmp(&y.1)
        } else {
            (y.2 as usize).cmp(&(x.2 as usize))
        }
    });

    let mut min_y = n + 1;
    for &(_x, y, c) in xyc.iter() {
        if c == 'B' {
            if y >= min_y {
                println!("No");
                return;
            }
        }
        if c == 'W' {
            // yに上限(下端)を侵食される
            min_y = min_y.min(y);
        }
    }

    println!("Yes");
}
