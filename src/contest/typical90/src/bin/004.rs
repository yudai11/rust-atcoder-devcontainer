use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize;w];h]
    }

    let mut b = vec![vec![0_usize; w]; h];
    let mut cul_sum = vec![0_usize; w];
    let mut row_sum = vec![0_usize; h];
    for i in 0..h {
        for j in 0..w {
            cul_sum[j] += a[i][j];
            row_sum[i] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            b[i][j] += cul_sum[j];
            b[i][j] += row_sum[i];
            b[i][j] -= a[i][j];
        }
    }

    println!("{}", b.iter().map(|bi| bi.iter().join(" ")).join("\n"));
}
