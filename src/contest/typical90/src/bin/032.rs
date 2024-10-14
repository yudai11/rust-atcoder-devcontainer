use itertools::Itertools;
use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m]
    }

    let mut ng_list = vec![vec![]; n];
    for &(x, y) in &xy {
        ng_list[x].push(y);
        ng_list[y].push(x);
    }

    let mut ans = 10001;

    for p in (0..n).permutations(n) {
        let mut pre_ans = 0;
        let mut feasi = true;
        for i in 0..(n - 1) {
            if ng_list[p[i]].contains(&p[i + 1]) {
                feasi = false;
                break;
            }
            pre_ans += a[p[i]][i];
        }
        if feasi {
            pre_ans += a[p[n - 1]][n - 1];
            ans = ans.min(pre_ans);
        }
    }

    if ans == 10001 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
