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

const DD: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars; h]
    }

    let mut ans = 0_usize;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }
            let mut seen = vec![vec![false; w]; h];
            let mut pre_ans = 0_usize;
            grid_dfs((i, j), (i, j), &c, h, w, &mut seen, &mut pre_ans, 1);
            ans = ans.max(pre_ans);
        }
    }

    if ans > 2 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn grid_dfs(
    start: (usize, usize),
    node: (usize, usize),
    c: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    seen: &mut Vec<Vec<bool>>,
    ans: &mut usize,
    depth: usize,
) {
    seen[node.0][node.1] = true;
    for i in 0..4 {
        if let Some(next) = move_grid(node.0, node.1, DD[i].0, DD[i].1, h, w) {
            if c[next.0][next.1] == '#' {
                continue;
            }
            if next == start {
                *ans = (*ans).max(depth);
            }
            if !seen[next.0][next.1] {
                grid_dfs(start, next, c, h, w, seen, ans, depth + 1);
                seen[next.0][next.1] = false;
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
