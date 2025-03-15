use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut conn_sets = vec![BinaryHeap::new(); n];

    for i in 0..n {
        conn_sets[i].push(i);
    }

    for _i in 0..q {
        input! {
            a: u8
        }
        match a {
            1 => {
                input! {
                    u: Usize1, v: Usize1
                }
                // uとvを含む連結集合の上から10点を持つheapを作る
                if dsu.same(u, v) {
                    continue;
                } else {
                    // uとvを含む連結集合の点(Max20点)を持つheap
                    let mut pre_new_heap = BinaryHeap::new();
                    // uの連結成分を追加
                    while let Some(p) = conn_sets[dsu.leader(u)].pop() {
                        pre_new_heap.push(p);
                    }
                    // vの連結成分を追加
                    while let Some(p) = conn_sets[dsu.leader(v)].pop() {
                        pre_new_heap.push(p);
                    }
                    // uとvを含む連結集合の上から10点を持つheap
                    let mut new_heap = BinaryHeap::new();
                    for _j in 0..10 {
                        if let Some(p) = pre_new_heap.pop() {
                            new_heap.push(p);
                        }
                    }
                    dsu.merge(u, v);
                    conn_sets[dsu.leader(u)] = new_heap;
                }
            }
            2 => {
                input! {
                    v: Usize1, k: usize
                }
                if conn_sets[dsu.leader(v)].len() < k {
                    println!("-1");
                } else {
                    let ans = *conn_sets[dsu.leader(v)].iter().nth(k - 1).unwrap();
                    println!("{}", ans + 1);
                }
            }
            _ => unreachable!(),
        }
    }
}
