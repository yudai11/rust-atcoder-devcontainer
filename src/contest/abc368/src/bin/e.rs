use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        x1: usize,
        abst: [(Usize1, Usize1, u64, u64); m]
    }

    let mut end_list = vec![vec![]; n];
    let mut start_list = vec![vec![]; n];

    for i in 0..m {
        start_list[abst[i].0].push(abst[i].2 + x1);
        end_list[abst[i].1].push(abst[i].3 + x1);
    }

    // 各駅で潰せる乗り換え時間を計算
    let mut gap: Vec<(usize, u64)> = vec![(0, 0); n];
    for i in 1..n {
        gap[i].0 = i;
        let min_sta_time = start_list[i].min();
        gap[i].1 = min_sta_time;
    }

    // 第2成分(min_start_time)でsort
    gap.sort_by_key(|x| x.1);

    let ans = vec![0; n];
    for i in 1..n {
        let num_station: usize = gap[i].0;
        let min_sta_time = gap[i].1;
        end_list[num_station].sort();
        end_list[num_station].reverse();
        let mut j = 0;
        let mut gap: u64 = 0;
        loop {
            if end_list[j] <= min_sta_time {
                gap = min_sta_time - end_list[j];
                break;
            }
            j += 1;
        }

        // for k in
    }

    println!("{:?}", a);
}
