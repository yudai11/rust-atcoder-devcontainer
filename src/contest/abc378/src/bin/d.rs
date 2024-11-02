use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        h: usize, w : usize, k: usize,
        s: [Chars; h]
    }

    let mut ans: usize = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut seen = vec![vec![false; w]; h];
            seen[i][j] = true;
            ans += dfs(i * w + j, &s, &mut seen, w, h, k, 0);
        }
    }

    println!("{ans}");
}

fn dfs(
    node: usize,
    s: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    w: usize,
    h: usize,
    k: usize,
    t: usize,
) -> usize {
    let dx: [isize; 4] = [0, 0, 1, -1];
    let dy: [isize; 4] = [1, -1, 0, 0];
    let (x, y) = (node / w, node % w);

    let mut res: usize = 0;

    if t == k {
        return 1;
    }

    for l in 0..4 {
        if x as isize + dx[l] < 0
            || x as isize + dx[l] >= h as isize
            || y as isize + dy[l] < 0
            || y as isize + dy[l] >= w as isize
        {
            continue;
        }
        let (x2, y2) = ((x as isize + dx[l]) as usize, (y as isize + dy[l]) as usize);

        if !seen[x2][y2] && s[x2][y2] == '.' {
            seen[x2][y2] = true;
            res += dfs(x2 * w + y2, s, seen, w, h, k, t + 1);
            seen[x2][y2] = false;
        }
    }

    return res;
}
