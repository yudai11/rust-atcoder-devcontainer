use std::collections::VecDeque;

use proconio::input;
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
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
         n: usize,
         h: [i64; n]
    }

    // let mut gap = vec![];
    // for i in 0..n-1 {
    //     gap.push(h[i+1] - h[i]);
    // }

    // let mut left = 0;
    // let mut right = 0;
    // while right < n {
    //     while gap[right - 1] > 0 {
    //         right += 1;
    //     }
    // }

    let mut ans = vec![0; n];
    let mut seen_buildings = VecDeque::new();

    for i in (0..n).rev() {
        ans[i] = seen_buildings.len();

        // 視点をビル(i-1)に変更
        if seen_buildings.is_empty() {
            seen_buildings.push_back(h[i]);
            continue;
        }

        let hi = h[i];
        while !seen_buildings.is_empty() && hi > seen_buildings[0] {
            let _ = seen_buildings.pop_front();
        }
        seen_buildings.push_front(hi);
    }

    for &x in ans.iter() {
        print!("{x} ");
    }
}
