use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars;h]
    }

    let mut start: usize = 0;
    let mut goal: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = i * w + j;
            }
            if s[i][j] == 'G' {
                goal = i * w + j;
            }
        }
    }

    let infty: usize = 1 << 60;
    let dx: [isize; 2] = [-1, 1];
    let dy: [isize; 2] = [-1, 1];
    // returnするvector
    let n = w * h;
    let mut dist_0 = vec![infty; n];
    let mut dist_1 = vec![infty; n];
    let mut seen_0 = vec![false; n];
    let mut seen_1 = vec![false; n];
    seen_0[start] = true;
    seen_1[start] = true;
    // 最大値が先頭に来るpriority queue
    let mut queue = BinaryHeap::new();
    // 0は横移動1は縦移動
    queue.push((Reverse(0), start, 0));
    queue.push((Reverse(0), start, 1));
    dist_0[start] = 0;
    dist_1[start] = 0;
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        let x = p.1 / w;
        let y = p.1 % w;
        if p.2 == 0 {
            for i in 0..2 {
                if x as isize + dx[i] >= 0 && x as isize + dx[i] < h as isize {
                    let new_x = (x as isize + dx[i]) as usize;
                    if seen_0[new_x * w + y] {
                        continue;
                    }
                    if s[new_x][y] == '#' {
                        continue;
                    }
                    seen_0[new_x * w + y] = true;
                    let to_v = dist_1[p.1] + 1;
                    if dist_0[new_x * w + y] > to_v {
                        dist_0[new_x * w + y] = to_v;
                        queue.push((Reverse(to_v), new_x * w + y, 1));
                    }
                }
            }
        }

        if p.2 == 1 {
            for i in 0..2 {
                if y as isize + dy[i] >= 0 && y as isize + dy[i] < w as isize {
                    let new_y = (y as isize + dy[i]) as usize;
                    if seen_1[x * w + new_y] {
                        continue;
                    }
                    if s[x][new_y] == '#' {
                        continue;
                    }
                    seen_1[x * w + new_y] = true;
                    let to_v = dist_0[p.1] + 1;
                    if dist_1[x * w + new_y] > to_v {
                        dist_1[x * w + new_y] = to_v;
                        queue.push((Reverse(to_v), x * w + new_y, 0));
                    }
                }
            }
        }
    }

    let ans = dist_0[goal].min(dist_1[goal]);
    if ans > 100000000000 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

// fn dijkstra(s: &Vec<Vec<char>>, n: usize, start: usize) -> Vec<usize> {
//     let infty: usize = 1 << 60;
//     let dx: [isize; 2] = [-1, 1];
//     let dy: [isize; 2] = [-1, 1];
//     // returnするvector
//     let mut dist = vec![infty; n];
//     let mut seen = vec![false; n];
//     // 最大値が先頭に来るpriority queue
//     let mut queue = BinaryHeap::new();
//     // 0は横移動1は縦移動
//     queue.push((Reverse(0), start, 0));
//     queue.push((Reverse(0), start, 1));
//     dist[start] = 0;
//     while !queue.is_empty() {
//         let p = queue.pop().unwrap();
//         let x = p.1 / w;
//         let y = p.1 % w;
//         if p.2 == 0 {
//             for i in 0..2 {
//                 if x as isize + dx[i] >= 0 && x as isize + dx[i] < h {
//                     let new_x = (x as isize + dx[i]) as usize;
//                     let to_v = dist[p.1] + 1 as u64;
//                     if dist[new_x * w + y] > to_v {
//                         dist[new_x * w + y] = to_v;
//                         queue.push((Reverse(to_v), new_x * w + y, 1));
//                     }
//                 }
//             }
//         }

//         if p.2 == 1 {
//             for i in 0..2 {
//                 if y as isize + dy[i] >= 0 && y as isize + dy[i] < w {
//                     let new_y = (y as isize + dy[i]) as usize;
//                     let to_v = dist[p.1] + 1 as u64;
//                     if dist[x * w + new_y] > to_v {
//                         dist[x * w + new_y] = to_v;
//                         queue.push((Reverse(to_v), x * w + new_y, 0));
//                     }
//                 }
//             }
//         }
//     }
//     dist
// }
