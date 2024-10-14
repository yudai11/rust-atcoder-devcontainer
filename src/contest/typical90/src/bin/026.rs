use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

struct Group {
    size: usize,
    val: Vec<usize>,
}

fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![vec![]; n];

    for _i in 0..(n - 1) {
        input! {
            (a,b) : (Usize1, Usize1)
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut group_0 = Group {
        size: 0,
        val: vec![],
    };
    let mut group_1 = Group {
        size: 0,
        val: vec![],
    };

    dfs(0, &graph, &mut seen, &mut group_0, &mut group_1, -1);

    if group_0.size > group_1.size {
        for i in 0..(n / 2) {
            print!("{} ", group_0.val[i] + 1);
        }
    } else {
        for i in 0..(n / 2) {
            print!("{} ", group_1.val[i] + 1);
        }
    }
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    group_0: &mut Group,
    group_1: &mut Group,
    index: isize,
) -> () {
    seen[node] = true;
    if index < 0 {
        group_0.size += 1;
        group_0.val.push(node);
    } else {
        group_1.size += 1;
        group_1.val.push(node);
    }
    for &next in graph[node].iter() {
        if !seen[next] {
            dfs(next, &graph, seen, group_0, group_1, -index)
        }
    }
}
