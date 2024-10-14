// use nalgebra::Matrix;
use proconio::{input, marker::Isize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
use petgraph::unionfind::UnionFind;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use proconio::fastout;

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut flag = vec![vec![false; w]; h];
    // let mut flag = Matrix::new(h,w,false);
    let mut uf = UnionFind::new(h * w);
    let dx: [isize; 4] = [1, 0, -1, 0];
    let dy: [isize; 4] = [0, 1, 0, -1];

    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! {
                mut a: (Isize1, Isize1),
            }
            // a.0 -= 1;
            // a.1 -= 1;
            flag[a.0 as usize][a.1 as usize] = true;
            for i in 0..4 {
                if a.0 + dy[i] >= 0
                    && a.1 + dx[i] >= 0
                    && a.0 + dy[i] < h as isize
                    && a.1 + dx[i] < w as isize
                    && flag[(a.0 + dy[i]) as usize][(a.1 + dx[i]) as usize]
                {
                    uf.union(
                        (a.0 * w as isize + a.1) as usize,
                        ((a.0 + dy[i]) * w as isize + a.1 + dx[i]) as usize,
                    );
                }
            }
        } else if t == 2 {
            input! {
                mut a: (Isize1, Isize1),
                mut b: (Isize1, Isize1),
            }
            if flag[a.0 as usize][a.1 as usize]
                && flag[b.0 as usize][b.1 as usize]
                && uf.find((a.0 * w as isize + a.1) as usize)
                    == uf.find((b.0 * w as isize + b.1) as usize)
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

// fn search_grid(i: &usize, h: &usize, w: &usize, a: &(usize, usize)) -> bool {
//     if a.0 + dy[i] >= 0 && a.1 + dx[i] >= 0 && a.0 + dy[i] < h as isize && a.1 + dx[i] < w as isize
//     {
//         return true;
//     } else {
//         return false;
//     }
// }
