use amplify::confinement::Collection;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
use std::collections::{BinaryHeap, HashSet};
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        h: usize, w: usize, zz: usize,
        p: Usize1, q: Usize1,
        s: [[usize;w];h]
    }

    let mut to_see: BinaryHeap<(Reverse<usize>, usize, usize, usize)> = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut ans: usize = s[p][q];
    seen.insert((h * p + q) as isize);

    let dx: [isize; 4] = [0, 0, 1, -1];
    let dy: [isize; 4] = [1, -1, 0, 0];

    for i in 0..4 {
        if p as isize + dx[i] < h as isize
            && 0 <= p as isize + dx[i]
            && q as isize + dy[i] < w as isize
            && 0 <= q as isize + dy[i]
        {
            let x_new = (p as isize + dx[i]) as usize;
            let y_new = (q as isize + dy[i]) as usize;
            if !seen.contains(&(h * x_new + y_new)) {
                to_see.push((
                    Reverse(s[x_new as usize][y_new as usize]),
                    s[x_new as usize][y_new as usize],
                    x_new,
                    y_new,
                ));

                seen.insert((h * x_new + y_new) as isize);
            }
        }
    }

    while !to_see.is_empty() {
        let (_p, point, x, y) = to_see.pop().unwrap();
        if point as usize > ans / zz {
            println!("{ans}");
            return;
        }
        ans += point as usize;

        for i in 0..4 {
            let x_new = x + dx[i];
            let y_new = y + dy[i];
            if x as isize + dx[i] < h as isize
                && 0 <= x as isize + dx[i]
                && y as isize + dy[i] < w as isize
                && 0 <= y as isize + dy[i]
            {
                if !seen.contains(&((h * x_new + y_new) as isize)) {
                    to_see.push((
                        Reverse(s[x_new as usize][y_new as usize]),
                        s[x_new as usize][y_new as usize],
                        x_new,
                        y_new,
                    ));

                    seen.insert((h * x_new + y_new) as isize);
                }
            }
        }
    }

    println!("{ans}");
}
