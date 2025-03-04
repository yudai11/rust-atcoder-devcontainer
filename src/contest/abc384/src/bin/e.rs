use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        h: usize, w: usize, x: usize,
        p: Usize1, q: Usize1,
        s: [[usize;w];h]
    }

    let mut ans = s[p][q];

    let dd: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut seen = vec![vec![false; w]; h];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0_usize), p, q));

    while let Some(u) = queue.pop() {
        seen[u.1][u.2] = true;
        let Reverse(val) = u.0;
        // オーバーフローに注意
        if val >= (ans + x - 1) / x {
            break;
        }
        ans += val;
        for i in 0..4 {
            if let Some(next) = move_grid(u.1, u.2, dd[i].0, dd[i].1, h, w) {
                if !seen[next.0][next.1] {
                    queue.push((Reverse(s[next.0][next.1]), next.0, next.1));
                    seen[next.0][next.1] = true;
                }
            }
        }
    }

    println!("{}", ans);
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
