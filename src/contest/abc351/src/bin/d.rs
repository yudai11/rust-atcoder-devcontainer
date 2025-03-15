// use im_rc::HashSet;
use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// use std::collections::BTreeSet;

const DD: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mut ans = 0_usize;
    let mut dp = vec![vec![0_usize; w]; h];
    for i in 0..h {
        for j in 0..w {
            // 移動ができるか
            let mut feasi = s[i][j] == '.';
            for k in 0..4 {
                if let Some((x, y)) = move_grid(i, j, DD[k].0, DD[k].1, h, w) {
                    feasi ^= !(s[x][y] == '.');
                }
            }
            if !feasi {
                continue;
            }
            // 自由度を求める．
            let mut pre_ans = 0_usize;
            for k in 0..4 {
                if let Some((x, y)) = move_grid(i, j, DD[k].0, DD[k].1, h, w) {
                    pre_ans = pre_ans.max(dp[x][y]);
                }
            }
            if pre_ans >= 0 {
                let mut seen = vec![vec![false; w]; h];
                seen[i][j] = true;
                dfs(i, j, &s, &mut seen, h, w, &mut pre_ans);
                ans = ans.max(pre_ans);
                dp[i][j] = pre_ans;
            }
        }
    }

    println!("{}", dp.iter().map(|x| x.iter().join(" ")).join("\n"));
    println!("{}", ans + 1);
}

fn dfs(
    i: usize,
    j: usize,
    s: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    h: usize,
    w: usize,
    res: &mut usize,
) {
    // 移動できるか
    for k in 0..4 {
        if let Some(v) = move_grid(i, j, DD[k].0, DD[k].1, h, w) {
            if s[v.0][v.1] == '#' {
                return;
            }
        }
    }
    for k in 0..4 {
        if let Some(v) = move_grid(i, j, DD[k].0, DD[k].0, h, w) {
            if !seen[v.0][v.1] && s[v.0][v.1] == '.' {
                seen[v.0][v.1] = true;
                dfs(i, j, s, seen, h, w, res);
                *res += 1;
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
