use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        _a: [usize;n],
        _b: [usize;n],
        _c: [usize;n],
    }
    let mut abc: [[usize; 46]; 3] = [[0; 46]; 3];

    for i in 0..n {
        abc[0][_a[i] % 46] += 1;
        abc[1][_b[i] % 46] += 1;
        abc[2][_c[i] % 46] += 1;
    }
    let mut ans: [[usize; 46]; 3] = [[0; 46]; 3];
    ans[0] = abc[0].clone();
    for k in 0..2 {
        for i in 0..46 {
            for j in 0..46 {
                ans[k + 1][(i + j) % 46] += ans[k][i] * abc[k + 1][j];
            }
        }
    }

    println!("{}", ans[2][0]);
}
