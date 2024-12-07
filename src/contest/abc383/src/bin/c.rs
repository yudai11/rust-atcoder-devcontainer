use std::collections::HashSet;

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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        h: usize, w: usize, d: isize,
        s: [Chars; h]
    }

    let mut t: Vec<Vec<isize>> = vec![vec![-1; w]; h];
    let mut to_see = HashSet::new();
    let mut ans: usize = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                t[i][j] = d + 1;
                ans += 1;
            } else if s[i][j] == '.' {
                t[i][j] = 0;
                to_see.insert(i * w + j);
            }
        }
    }

    let dx: [isize; 4] = [0, 0, 1, -1];
    let dy: [isize; 4] = [1, -1, 0, 0];

    for _i in 0..d as usize {
        let mut remove_list = vec![];
        for &v in to_see.iter() {
            let (x, y) = ((v / w) as isize, (v % w) as isize);
            for i in 0..4 {
                if x + dx[i] < h as isize
                    && 0 <= x + dx[i]
                    && y + dy[i] < w as isize
                    && 0 <= y + dy[i]
                {
                    let q = t[(x + dx[i]) as usize][(y + dy[i]) as usize];
                    if q > 1 {
                        t[x as usize][y as usize] = t[x as usize][y as usize].max(q - 1);
                    }
                }
            }
            if t[x as usize][y as usize] > 0 {
                remove_list.push(v);
            }
        }
        for v in remove_list.iter() {
            to_see.remove(&v);
            ans += 1;
        }
    }

    println!("{ans}");
}
