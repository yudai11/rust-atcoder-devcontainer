use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    }

    let mut num_small_nbh = vec![0; n];
    for i in 0..m {
        let &(a, b) = &edges[i];
        if a < b {
            num_small_nbh[b] += 1;
        } else {
            num_small_nbh[a] += 1;
        }
    }

    println!(
        "{}",
        num_small_nbh
            .iter()
            .fold(0, |ans, &x| ans + if x == 1 { 1 } else { 0 })
    );
}
