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
        n: usize
    }

    let mut grid = vec![vec![0_isize; n]; n];
    grid[n / 2][n / 2] = -1;
    let mut ind = 1_isize;

    let mut loc = Information {
        x: 0,
        y: 0,
        direction: 0,
    };

    grid[loc.y][loc.x] = ind;
    let mut stuck_point = 0_usize;

    while stuck_point < 6 {
        let t = loc.direction;
        match t {
            0 => {
                if loc.x + 1 < n && grid[loc.y][loc.x + 1] == 0 {
                    stuck_point = 0;
                    loc.x += 1;
                    ind += 1;
                    grid[loc.y][loc.x] = ind;
                } else {
                    loc.direction += 1;
                    stuck_point += 1;
                }
            }
            1 => {
                if loc.y > 0 && grid[loc.y - 1][loc.x] == 0 {
                    stuck_point = 0;
                    loc.y -= 1;
                    ind += 1;
                    grid[loc.y][loc.x] = ind;
                } else {
                    loc.direction += 1;
                    stuck_point += 1;
                }
            }
            2 => {
                if loc.x > 0 && grid[loc.y][loc.x - 1] == 0 {
                    stuck_point = 0;
                    loc.x -= 1;
                    ind += 1;
                    grid[loc.y][loc.x] = ind;
                } else {
                    loc.direction += 1;
                    stuck_point += 1;
                }
            }
            3 => {
                if loc.y + 1 < n && grid[loc.y + 1][loc.x] == 0 {
                    stuck_point = 0;
                    loc.y += 1;
                    ind += 1;
                    grid[loc.y][loc.x] = ind;
                } else {
                    loc.direction = 0;
                    stuck_point += 1;
                }
            }
            _ => {
                println!("Err");
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] < 0 {
                print!("T ");
            } else {
                print!("{} ", grid[i][j]);
            }
        }
        println!("");
    }
}

struct Information {
    x: usize,
    y: usize,
    direction: usize,
}
