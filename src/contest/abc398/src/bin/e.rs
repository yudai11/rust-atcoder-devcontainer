use proconio::source::line::LineSource;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;
use std::io::{self, BufReader};

fn main() {
    // インタラクティブな問題でのinput の設定
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
        edges: [(Usize1,Usize1); n-1]
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut seen = vec![false; n];
    let mut gp_1 = vec![];
    let mut gp_2 = vec![];
    make_bipartite_graph(0, &graph, &mut seen, &mut gp_1, &mut gp_2, true);
    let mut num_can_draw_edge = vec![HashSet::new(); n];
    for &u in gp_1.iter() {
        for &v in gp_2.iter() {
            num_can_draw_edge[u].insert(v);
        }
        for &v in graph[u].iter() {
            num_can_draw_edge[u].remove(&v);
        }
    }
    for &u in gp_2.iter() {
        for &v in gp_1.iter() {
            num_can_draw_edge[u].insert(v);
        }
        for &v in graph[u].iter() {
            num_can_draw_edge[u].remove(&v);
        }
    }

    let sum = num_can_draw_edge
        .iter()
        .fold(0_usize, |sum, x| sum + x.len());
    if (sum / 2) % 2 != 0 {
        println!("First");
        for i in 0..n {
            if let Some(j) = num_can_draw_edge[i].iter().next().cloned() {
                println!("{} {}", i + 1, j + 1);
                num_can_draw_edge[i].remove(&j);
                num_can_draw_edge[j].remove(&i);
                break;
            }
        }

        loop {
            input! {
                (x,y): (isize,isize)
            }
            if x == -1 {
                return;
            }

            let x = (x - 1) as usize;
            let y = (y - 1) as usize;

            num_can_draw_edge[x].remove(&y);
            num_can_draw_edge[y].remove(&x);

            for i in 0..n {
                if let Some(j) = num_can_draw_edge[i].iter().next().cloned() {
                    println!("{} {}", i + 1, j + 1);
                    num_can_draw_edge[i].remove(&j);
                    num_can_draw_edge[j].remove(&i);
                    break;
                }
            }
        }
    } else {
        println!("Second");

        loop {
            input! {
                (x,y): (isize,isize)
            }
            if x == -1 && y == -1 {
                return;
            }

            let x = (x - 1) as usize;
            let y = (y - 1) as usize;

            num_can_draw_edge[x].remove(&y);
            num_can_draw_edge[y].remove(&x);

            for i in 0..n {
                if let Some(j) = num_can_draw_edge[i].iter().next().cloned() {
                    println!("{} {}", i + 1, j + 1);
                    num_can_draw_edge[i].remove(&j);
                    num_can_draw_edge[j].remove(&i);
                    break;
                }
            }
        }
    }
}

fn make_bipartite_graph(
    node: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    gp_1: &mut Vec<usize>,
    gp_2: &mut Vec<usize>,
    state: bool,
) {
    seen[node] = true;
    if state {
        gp_1.push(node);
    } else {
        gp_2.push(node);
    }
    for &next in graph[node].iter() {
        if !seen[next] {
            make_bipartite_graph(next, graph, seen, gp_1, gp_2, !state)
        }
    }
}
