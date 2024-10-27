use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: u128,
        p: [Usize1; n]
    }

    // k を 2 進数で表現
    let mut k_mod2: Vec<usize> = vec![];
    let mut m = k;
    while m > 0 {
        k_mod2.push((m % 2) as usize);
        m /= 2;
    }

    let m = k_mod2.len();
    // ps 配列に 2 のべき乗回分の置換を保持
    let mut ps: Vec<Vec<usize>> = vec![vec![0; n]; m];
    ps[0] = p.clone();
    for i in 1..m {
        ps[i] = mat_time(&ps[i - 1], &ps[i - 1], n);
    }

    let mut ans: Vec<usize> = p.clone();

    // k の各ビットに応じて、置換を合成
    for i in 0..m {
        if k_mod2[i] != 0 {
            ans = mat_time(&ans, &ans, n);
        }
    }

    for i in 0..n {
        print!("{} ", ans[i] + 1);
    }
}

// mat_time: 2 つの置換を合成する関数
fn mat_time(x: &Vec<usize>, y: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut z: Vec<usize> = vec![0; n];
    for i in 0..n {
        z[i] = x[y[i]];
    }
    z
}
