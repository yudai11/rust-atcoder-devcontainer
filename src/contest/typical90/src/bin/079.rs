use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h: usize, w: usize,
        mut a: [[isize;w];h],
        b: [[isize;w];h],
    }

    let sum_a = a
        .iter()
        .fold(0, |sum, x| sum + x.iter().fold(0, |sum, &x| sum + x));
    let sum_b = b
        .iter()
        .fold(0, |sum, x| sum + x.iter().fold(0, |sum, &x| sum + x));

    if (sum_a - sum_b) % 4 != 0 {
        println!("No");
    } else {
        let mut cnt = 0;
        for i in 0..h {
            for j in 0..w {
                let gap = b[i][j] - a[i][j];
                cnt += gap.abs();
                a[i][j] += gap;
                if i + 1 < h {
                    a[i + 1][j] += gap;
                }
                if j + 1 < w {
                    a[i][j + 1] += gap;
                }
                if i + 1 < h && j + 1 < w {
                    a[i + 1][j + 1] += gap;
                }
            }
        }

        println!("Yes");
        println!("{cnt}");
    }
}
