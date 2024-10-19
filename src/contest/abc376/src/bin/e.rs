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
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize, k: usize,
            a: [usize;n], b: [usize;n]
        }

        let mut ab: Vec<(usize, usize)> = vec![];
        for i in 0..n {
            ab.push((a[i], b[i]));
        }
        ab.sort_by(|&x, &y| x.0.cmp(&y.0)); // Aを基準に昇順でソート

        // Bの値に基づいて最大K個の部分集合を選ぶためにヒープを使う
        let mut heap = BinaryHeap::new();
        let mut sum_b = 0;
        let mut min_value = usize::MAX;

        // K個選んでその時の結果を計算
        for i in 0..n {
            let (a_i, b_i) = ab[i];
            heap.push(b_i);
            sum_b += b_i;

            if heap.len() > k {
                if let Some(max_b) = heap.pop() {
                    sum_b -= max_b;
                }
            }

            if heap.len() == k {
                min_value = min_value.min(a_i * sum_b);
            }
        }

        println!("{}", min_value);
    }
}
