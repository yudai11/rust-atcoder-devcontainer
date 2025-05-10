use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

const DD: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    let infty: usize = 1000_000_000_000 as usize;
    // returnするvector
    let mut dist = vec![vec![infty; w]; h];
    let mut direction = vec![vec![4_usize; w]; h];
    // 最大値が先頭に来るpriority queue
    let mut queue: BinaryHeap<_> = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                queue.push((Reverse(0), i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some(p) = queue.pop() {
        for i in 0..4 {
            if let Some(next) = move_grid(p.1, p.2, DD[i].0, DD[i].1, h, w) {
                if s[next.0][next.1] != '.' {
                    continue;
                }
                let to_next = dist[p.1][p.2] + 1;
                if dist[next.0][next.1] > to_next {
                    dist[next.0][next.1] = to_next;
                    direction[next.0][next.1] = i;
                    queue.push((Reverse(to_next), next.0, next.1));
                }
            }
        }
    }

    let mut t = vec![vec!['1'; w]; h];

    for i in 0..h {
        for j in 0..w {
            match direction[i][j] {
                0 => {
                    t[i][j] = '^';
                }
                1 => {
                    t[i][j] = '<';
                }
                2 => {
                    t[i][j] = 'v';
                }
                3 => {
                    t[i][j] = '>';
                }
                _ => {
                    t[i][j] = s[i][j];
                }
            }
        }
    }

    println!("{}", t.iter().map(|x| x.iter().join("")).join("\n"));
}

// fn dijkstra(
//     s: &Vec<Vec<char>>,
//     h: usize,
//     w: usize,
//     start: (usize, usize),
//     direction: usize,
// ) -> Vec<Vec<usize>> {
//     let infty: usize = 1000_000_000_000 as usize;
//     // returnするvector
//     let mut dist = vec![vec![(infty, 5_usize); w]; h];
//     // 最大値が先頭に来るpriority queue
//     let mut queue = BinaryHeap::new();
//     queue.push((Reverse(0), start.0, start.1, direction));
//     dist[start.0][start.1] = 0;
//     while let Some(p) = queue.pop() {
//         for i in 0..4 {
//             if let Some(next) = move_grid(p.1, p.2, DD[i].0, DD[i].1, h, w) {
//                 if i == direction {}
//             }
//         }
//         for &v in graph[p.1].iter() {
//             let to_v = dist[p.1] + v.1 as usize;
//             if dist[v.0] > to_v {
//                 dist[v.0] = to_v;
//                 queue.push((Reverse(to_v), v.0));
//             }
//         }
//     }
//     dist
// }

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
