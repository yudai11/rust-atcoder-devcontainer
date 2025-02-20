use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut val: usize = 0;
    let mut seen = HashSet::new();
    dfs(n, 0, 0, &mut vec![0_usize; n], &mut seen, &a, &mut val);

    println!("{}", seen.len());
}

// 状態dfs
fn dfs(
    n: usize,
    depth: usize,
    num_gr: usize,
    sums: &mut Vec<usize>,
    seen: &mut HashSet<usize>,
    a: &Vec<usize>,
    val: &mut usize,
) {
    for i in 0..=num_gr {
        *val ^= sums[i];
        sums[i] += a[depth];
        *val ^= sums[i];
        if depth == n - 1 {
            seen.insert(*val);
        } else if i == num_gr {
            dfs(n, depth + 1, num_gr + 1, sums, seen, a, val)
        } else {
            dfs(n, depth + 1, num_gr, sums, seen, a, val)
        }
        // 撤収作業
        *val ^= sums[i];
        sums[i] -= a[depth];
        *val ^= sums[i];
    }
}
