use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

// imos法
fn main() {
    input! {
        n: usize,
        coords: [(usize,usize,usize,usize);n]
    }

    let mut plane: [[i64; 1002]; 1002] = [[0; 1002]; 1002];
    let mut ans: Vec<i32> = vec![0; n + 1];

    for i in 0..n {
        let (a, b, c, d) = coords[i];
        plane[a][b] += 1;
        plane[a][d] -= 1;
        plane[c][b] -= 1;
        plane[c][d] += 1;
    }

    for i in 0..=1000 {
        for j in 1..=1000 {
            plane[j][i] += plane[j - 1][i];
        }
    }

    for i in 0..=1000 {
        for j in 1..=1000 {
            plane[i][j] += plane[i][j - 1];
        }
    }

    for i in 0..=1000 {
        for j in 0..=1000 {
            ans[plane[i][j] as usize] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
