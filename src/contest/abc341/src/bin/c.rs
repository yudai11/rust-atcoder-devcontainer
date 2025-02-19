use std::collections::HashSet;

use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
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
        h: usize, w: usize, _n: usize,
        t: Chars,
        s: [Chars; h]
    }

    let mut ans = 0_usize;
    let mut seen = HashSet::new();
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if s[i][j] == '#' {
                continue;
            }
            match move_func(i, j, &t, &s) {
                Some((x, y)) => {
                    if !seen.contains(&(x, y)) {
                        ans += 1;
                        seen.insert((x, y));
                    }
                }
                None => {}
            }
        }
    }

    println!("{}", ans);
}

fn move_func(p: usize, q: usize, t: &Vec<char>, s: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut x = p;
    let mut y = q;
    for &ti in t.iter() {
        match ti {
            'L' => {
                if s[x][y - 1] == '#' {
                    return None;
                } else {
                    y -= 1;
                }
            }
            'R' => {
                if s[x][y + 1] == '#' {
                    return None;
                } else {
                    y += 1;
                }
            }
            'U' => {
                if s[x - 1][y] == '#' {
                    return None;
                } else {
                    x -= 1;
                }
            }
            'D' => {
                if s[x + 1][y] == '#' {
                    return None;
                } else {
                    x += 1;
                }
            }
            _ => {}
        }
    }

    if s[x][y] == '#' {
        return None;
    } else {
        return Some((x, y));
    }
}
