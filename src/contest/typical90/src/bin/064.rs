use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, q: usize,
        a: [isize;n],
        lrv: [(Usize1, Usize1, isize);q]
    }

    let mut height_gap: Vec<isize> = vec![0; n - 1];
    for i in 0..(n - 1) {
        height_gap[i] = a[i + 1] - a[i];
    }

    let mut sum: isize = height_gap.iter().fold(0, |sum, &x| sum + x.abs());

    for &(l, r, v) in &lrv {
        if l > 0 {
            sum -= height_gap[(l - 1) as usize].abs() as isize;
            height_gap[(l - 1) as usize] += v;
            sum += height_gap[(l - 1) as usize].abs() as isize;
        }
        if r < n - 1 {
            sum -= height_gap[r].abs() as isize;
            height_gap[r] -= v;
            sum += height_gap[r].abs() as isize;
        }

        println!("{}", sum);
    }
}
