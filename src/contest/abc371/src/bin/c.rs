use std::{collections::HashSet, usize};

use itertools::Itertools;
use proconio::{input, marker::Usize1};
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
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mg: usize,
        g_edges: [(Usize1,Usize1);mg],
        mh: usize,
        h_edges: [(Usize1,Usize1);mh],
        _a: [usize; n * (n-1) / 2]
    }

    let mut a = vec![vec![0; n]; n - 1];
    let mut k = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            a[i][j] = _a[k];
            k += 1;
        }
    }

    let mut g = vec![HashSet::new(); n];
    let mut h = vec![HashSet::new(); n];
    for i in 0..mg {
        let (v, w) = g_edges[i];
        g[v].insert(w);
        g[w].insert(v);
    }
    for i in 0..mh {
        let (v, w) = h_edges[i];
        h[v].insert(w);
        h[w].insert(v);
    }

    let mut ans = usize::MAX;
    for perm in (0..n).permutations(n) {
        let mut pre_ans: usize = 0;
        let mut h2 = h.clone();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if g[i].contains(&j) && !h2[perm[i]].contains(&perm[j]) {
                    h2[perm[i]].insert(perm[j]);
                    h2[perm[j]].insert(perm[i]);
                    if perm[i] < perm[j] {
                        pre_ans += a[perm[i]][perm[j]];
                    } else {
                        pre_ans += a[perm[j]][perm[i]];
                    }
                } else if !g[i].contains(&j) && h2[perm[i]].contains(&perm[j]) {
                    h2[perm[i]].remove(&perm[j]);
                    h2[perm[j]].remove(&perm[i]);
                    if perm[i] < perm[j] {
                        pre_ans += a[perm[i]][perm[j]];
                    } else {
                        pre_ans += a[perm[j]][perm[i]];
                    }
                }
            }
        }

        ans = ans.min(pre_ans);
    }

    println!("{ans}");
}
