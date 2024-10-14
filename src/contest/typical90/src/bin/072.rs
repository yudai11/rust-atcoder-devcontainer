use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars;h]
    }

    let mut ans = -1;

    for x in 0..h {
        for y in 0..w {
            if c[x][y] == '.' {
                ans = ans.max(search_railway(x, y, &c, h, w));
            }
        }
    }

    println!("{:?}", a);
}

// Calculate the maximum value of the circumference from each point
fn search_railway(x: usize, y: usize, c: &Vec<Vec<char>>, h: usize, w: usize) -> isize {
    let mut ans = -1;
    let mut seen = vec![vec![false; w]; h];
    let ans = dfs((x, y), c, &mut seen, h, w);
    1
}

fn dfs(
    node: (usize, usize),
    c: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    h: usize,
    w: usize,
) -> isize {
    seen[node] = true;
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs(next, &graph, seen, h, w);
        }
    }
    1
}
