use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        coordinates: [(usize,usize,usize,usize); n]
    }

    let mut field = vec![vec![0_isize; 1003]; 1003];

    for &(l1, l2, r1, r2) in coordinates.iter() {
        field[l1][l2] += 1;
        field[l1][r2] -= 1;
        field[r1][l2] -= 1;
        field[r1][r2] += 1;
    }

    for i in 0..=1000 {
        for j in 1..=1000 {
            field[i][j] += field[i][j - 1];
        }
    }

    for i in 0..=1000 {
        for j in 1..=1000 {
            field[j][i] += field[j - 1][i];
        }
    }

    let mut ans = vec![0_usize; n];
    for i in 0..=1000 {
        for j in 0..=1000 {
            if field[i][j] > 0 {
                ans[field[i][j] as usize - 1] += 1;
            }
        }
    }

    println!("{}", ans.iter().join("\n"));
}
