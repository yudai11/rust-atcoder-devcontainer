use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
// use itertools::Itertools;
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

const DD: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
        start: (Usize1,Usize1),
        goal: (Usize1,Usize1),
    }

    let dist = dijkstra(&s, h, w, start.0 * w + start.1);
    println!("{}", dist[goal.0 * w + goal.1]);
}

fn dijkstra(s: &Vec<Vec<char>>, h: usize, w: usize, start: usize) -> Vec<usize> {
    let infty: usize = 1000_000_000_000_000_000 as usize;
    // returnするvector
    let mut dist = vec![infty; h * w];
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    dist[start] = 0;
    while let Some(p) = queue.pop() {
        for i in 0..4 {
            if let Some(v) = move_grid(p.1 / w, p.1 % w, DD[i].0, DD[i].1, h, w) {
                if s[v.0][v.1] == '.' {
                    let to_v = dist[p.1];
                    if dist[v.0 * w + v.1] > to_v {
                        dist[v.0 * w + v.1] = to_v;
                        queue.push((Reverse(to_v), v.0 * w + v.1));
                    }
                } else {
                    let to_v = dist[p.1] + 1;
                    if dist[v.0 * w + v.1] > to_v {
                        dist[v.0 * w + v.1] = to_v;
                        queue.push((Reverse(to_v), v.0 * w + v.1));
                    }
                }
            }
            if let Some(v) = move_grid(p.1 / w, p.1 % w, DD[i].0 * 2, DD[i].1 * 2, h, w) {
                if s[v.0][v.1] == '#' {
                    let to_v = dist[p.1] + 1;
                    if dist[v.0 * w + v.1] > to_v {
                        dist[v.0 * w + v.1] = to_v;
                        queue.push((Reverse(to_v), v.0 * w + v.1));
                    }
                }
            }
        }
    }
    dist
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
