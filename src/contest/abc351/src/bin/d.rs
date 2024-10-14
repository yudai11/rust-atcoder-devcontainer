// use im_rc::HashSet;
use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// use std::collections::BTreeSet;

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    // let move_command = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut seen = vec![false; w * h];
    let mut ans = -1;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(dfs(i, j, i, j, &s, &mut seen, h, w));
        }
    }
}

fn dfs(
    i: usize,
    j: usize,
    x: usize,
    y: usize,
    s: &Vec<Vec<char>>,
    seen: &mut Vec<bool>,
    h: usize,
    w: usize,
) -> isize {
    seen[i * w + j] = true;
    let move_command = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut v: isize = -10000;
    for i in 0..4 {
        let (dr, dc) = move_command[i];
        let r = x as isize + dr;
        let c = y as isize + dc;
        if r < 0 || r >= h as isize || c < 0 || c >= w as isize {
            continue;
        }
        if r as usize == x && c as usize == y {
            v = 0;
        }
        if !seen[i * w + j] {
            v = dfs(i, j, x, y, s, seen, h, w) + 1;
        }
    }
    // seen[i * w + j] = false;
    if v < 0 {
        v = -10000;
    }
    v
}

// fn no_mag_in_nbh(x: usize, y: usize, c: &Vec<Vec<char>>, h: usize, w: usize) -> bool {
//     let mut can = true;
//     let move_command = [(0, 1), (1, 0), (0, -1), (-1, 0)];
//     for i in 0..4 {
//         let (dr, dc) = move_command[i];
//         let x = x as isize + dr;
//         let y = y as isize + dc;
//         if x < 0 || x >= h as isize || y < 0 || y >= w as isize {
//             continue;
//         }
//         if c[x as usize][y as usize] == '#' {
//             can = false;
//             break;
//         }
//     }
//     // if can {
//     //     println!("can: ");
//     // } else {
//     //     println!("cannot: ");
//     // }
//     return can;
// }
