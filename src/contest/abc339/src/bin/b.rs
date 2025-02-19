use itertools::Itertools;
use proconio::input;
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
        h: usize, w: usize, n: usize
    }

    let mut grid = vec![vec!['.'; w]; h];
    let mut loc = Location {
        x: 0_usize,
        y: 0_usize,
        direction: 0_usize,
    };
    for _i in 0..n {
        if grid[loc.y][loc.x] == '.' {
            grid[loc.y][loc.x] = '#';
            loc.direction += 1;
            loc.direction %= 4;
            match loc.direction {
                0 => {
                    loc.y += h - 1;
                    loc.y %= h;
                }
                1 => {
                    loc.x += 1;
                    loc.x %= w;
                }
                2 => {
                    loc.y += 1;
                    loc.y %= h;
                }
                3 => {
                    loc.x += w - 1;
                    loc.x %= w;
                }
                _ => {}
            }
        } else {
            grid[loc.y][loc.x] = '.';
            loc.direction += 3;
            loc.direction %= 4;
            match loc.direction {
                0 => {
                    loc.y += h - 1;
                    loc.y %= h;
                }
                1 => {
                    loc.x += 1;
                    loc.x %= w;
                }
                2 => {
                    loc.y += 1;
                    loc.y %= h;
                }
                3 => {
                    loc.x += w - 1;
                    loc.x %= w;
                }
                _ => {}
            }
        }
    }

    for i in 0..h {
        println!("{}", grid[i].iter().join(""));
    }
}

struct Location {
    x: usize,
    y: usize,
    direction: usize,
}
