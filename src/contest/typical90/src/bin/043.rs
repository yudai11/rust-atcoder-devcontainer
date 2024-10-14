// use pathfinding::directed::bfs;
// use ac_library::Monoid;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h: usize, w: usize,
        s: (Usize1, Usize1),
        t: (Usize1, Usize1),
        g: [Chars; h],
    }
    const INF: usize = usize::MAX;
    const DIR: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut costs = vec![vec![[INF; 4]; w]; h];

    // Set the cost of the starting point to 0
    costs[s.0][s.1] = [0, 0, 0, 0];

    // VecDeque has structure both stack and queue
    let mut deq = VecDeque::new();
    for di in 0..4 {
        deq.push_back((s.0, s.1, di));
    }
    while let Some((hi, wi, di)) = deq.pop_front() {
        // Search in 4 directions
        for (ndi, &(dh, dw)) in DIR.iter().enumerate() {
            let nhi = hi.wrapping_add_signed(dh);
            let nwi = wi.wrapping_add_signed(dw);
            if nhi < h && nwi < w && g[nhi][nwi] == '.' {
                // Increase costs if direction is changed
                let samedir = di == ndi;
                let ncost = costs[hi][wi][di] + if samedir { 0 } else { 1 };

                if ncost < costs[nhi][nwi][ndi] {
                    costs[nhi][nwi][ndi] = ncost;
                    // Search order backward if direction changed, forward if not.
                    // Why?

                    if samedir {
                        deq.push_front((nhi, nwi, ndi));
                    } else {
                        deq.push_back((nhi, nwi, ndi));
                    }
                }
            }
        }
    }
    let res = costs[t.0][t.1].iter().min().unwrap();
    println!("{}", res);
}
