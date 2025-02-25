use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
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
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mut ans = 0_usize;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' || seen.contains(&(i, j)) {
                continue;
            }
            ans += 1;
            dfs((i, j), &s, &mut seen, h, w);
        }
    }

    println!("{ans}");
}

fn dfs(
    node: (usize, usize),
    s: &Vec<Vec<char>>,
    seen: &mut HashSet<(usize, usize)>,
    h: usize,
    w: usize,
) {
    seen.insert(node);
    let d: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    for i in 0..8 {
        if let Some(v) = move_grid(node.0, node.1, d[i].0, d[i].1, h, w) {
            if s[v.0][v.1] == '#' && !seen.contains(&v) {
                dfs(v, s, seen, h, w);
            }
        }
    }
}

// gridの移動ができるならばその中身を返す関数
fn move_grid(
    i: usize,
    j: usize,
    dx: isize,
    dy: isize,
    h: usize,
    w: usize,
) -> Option<(usize, usize)> {
    if i as isize + dx >= 0
        && i as isize + dx < h as isize
        && j as isize + dy >= 0
        && j as isize + dy < w as isize
    {
        return Some(((i as isize + dx) as usize, (j as isize + dy) as usize));
    } else {
        return None;
    }
}
