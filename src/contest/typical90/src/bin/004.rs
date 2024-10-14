use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut sum_row = vec![0; h];
    let mut sum_column = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_row[i] += a[i][j];
            sum_column[j] += a[i][j];
        }
    }

    let mut b = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            b[i][j] = sum_row[i] + sum_column[j] - a[i][j];
            print!("{} ", b[i][j]);
        }
        println!("");
    }
}
