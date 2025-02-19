use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    }

    let mut ans = 1000_000_000_00 as usize;

    let dx: [isize; 4] = [0, 0, 1, -1];
    let dy: [isize; 4] = [1, -1, 0, 0];

    // Playerの位置を特定
    let mut starts = vec![];
    for i in 0..n * n {
        if s[i / n][i % n] == 'P' {
            starts.push(i);
        }
    }

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((starts[0], starts[1], 0_usize));
    seen.insert((starts[0], starts[1]));

    while let Some(v) = queue.pop_front() {
        let p = (v.0 / n, v.0 % n);
        let q = (v.1 / n, v.1 % n);
        for k in 0..4 {
            let mut new_i = v.0;
            let mut new_j = v.1;
            if p.0 as isize + dy[k] >= 0 && p.0 as isize + dy[k] < n as isize {
                if s[(p.0 as isize + dy[k]) as usize][p.1] != '#' {
                    new_i = (new_i as isize + dy[k] * n as isize) as usize;
                }
            }
            if q.0 as isize + dy[k] >= 0 && q.0 as isize + dy[k] < n as isize {
                if s[(q.0 as isize + dy[k]) as usize][q.1] != '#' {
                    new_j = (new_j as isize + dy[k] * n as isize) as usize;
                }
            }
            if p.1 as isize + dx[k] >= 0 && p.1 as isize + dx[k] < n as isize {
                if s[p.0][(p.1 as isize + dx[k]) as usize] != '#' {
                    new_i = (new_i as isize + dx[k]) as usize;
                }
            }
            if q.1 as isize + dx[k] >= 0 && q.1 as isize + dx[k] < n as isize {
                if s[q.0][(q.1 as isize + dx[k]) as usize] != '#' {
                    new_j = (new_j as isize + dx[k]) as usize;
                }
            }

            if (new_i != v.0 || new_j != v.1) && !seen.contains(&(new_i, new_j)) {
                queue.push_back((new_i, new_j, v.2 + 1));
                seen.insert((new_i, new_j));
                if new_i == new_j {
                    ans = ans.min(v.2 + 1);
                }
            }
        }
    }

    // for i in 0..n * n {
    //     for j in 0..n * n {
    //         let p = (i / n, i % n);
    //         let q = (j / n, j % n);
    //         for k in 0..4 {
    //             let mut new_i = i;
    //             let mut new_j = j;
    //             if p.0 as isize + dy[k] >= 0 && p.0 as isize + dy[k] < n as isize {
    //                 if s[(p.0 as isize + dy[k]) as usize][p.1] != '#' {
    //                     new_i = (new_i as isize + dy[k] * n as isize) as usize;
    //                 }
    //             }
    //             if q.0 as isize + dy[k] >= 0 && q.0 as isize + dy[k] < n as isize {
    //                 if s[(q.0 as isize + dy[k]) as usize][q.1] != '#' {
    //                     new_j = (new_j as isize + dy[k] * n as isize) as usize;
    //                 }
    //             }
    //             if p.1 as isize + dx[k] >= 0 && p.1 as isize + dx[k] < n as isize {
    //                 if s[p.0][(p.1 as isize + dx[k]) as usize] != '#' {
    //                     new_i = (new_i as isize + dx[k]) as usize;
    //                 }
    //             }
    //             if q.1 as isize + dx[k] >= 0 && q.1 as isize + dx[k] < n as isize {
    //                 if s[q.0][(q.1 as isize + dx[k]) as usize] != '#' {
    //                     new_j = (new_j as isize + dx[k]) as usize;
    //                 }
    //             }

    //             if new_i != i || new_j != j {
    //                 graph[i * n * n + j].push((new_i * n * n + new_j, 1))
    //             }
    //         }
    //     }
    // }

    // let dist_list = dijkstra(&graph, n * n * n * n, starts[0] * n * n + starts[1]);
    // let mut ans = 1000_000_000_00_usize;
    // for i in 0..n * n {
    //     ans = ans.min(dist_list[i * n * n + i]);
    // }
    if ans < 1000_000_000 {
        println!("{}", ans);
    } else {
        println!("-1");
    }

    // 検証用
    // match n {
    //     5 => {
    //         println!("3");
    //     }
    //     2 => {
    //         println!("-1");
    //     }
    //     10 => {
    //         println!("10");
    //     }
    //     _ => {}
    // }
}

// fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, n: usize, start: usize) -> Vec<usize> {
//     let infty: usize = 1000_000_000_00;
//     // returnするvector
//     let mut dist = vec![infty; n];
//     let mut seen = HashSet::new();
//     // 最大値が先頭に来るpriority queue
//     let mut queue = BinaryHeap::new();
//     queue.push((Reverse(0), start));
//     seen.insert(start);
//     dist[start] = 0;
//     while !queue.is_empty() {
//         let p = queue.pop().unwrap();
//         for &v in graph[p.1].iter() {
//             if seen.contains(&v.0) {
//                 continue;
//             }
//             let to_v = dist[p.1] + v.1 as usize;
//             if dist[v.0] > to_v {
//                 dist[v.0] = to_v;
//                 queue.push((Reverse(to_v), v.0));
//                 seen.insert(v.0);
//             }
//         }
//     }
//     dist
// }
