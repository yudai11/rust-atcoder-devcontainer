use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize, w: usize,
        xy: [(Usize1,Usize1); n],
        q: usize,
    }

    let mut columun_elements = vec![vec![]; w];
    for &(x, y) in xy.iter() {
        columun_elements[x].push(y);
    }

    let mut min_len = n;
    for i in 0..w {
        min_len = min_len.min(columun_elements[i].len());
        columun_elements[i].sort();
    }

    // //初手に最下段が消えるか
    // let mut do_vanish_first = true;
    // for i in 0..w {
    //     if columun_elements[i][0] != 0 {
    //         do_vanish_first = false;
    //         break;
    //     }
    // }

    // 最下段が消滅する時間を記録
    let mut vanish_times = vec![1000_000_000_000_000; n];
    for i in 0..min_len {
        let mut res = 0_usize;
        for j in 0..w {
            res = res.max(columun_elements[j][i]);
        }
        vanish_times[i] = res + 1;
    }

    //queryの回答
    for _i in 0..q {
        input! {
            t: usize, a: Usize1
        }
        let (x, y) = xy[a];
        let loc = columun_elements[x].lower_bound(&y);
        if vanish_times[loc] <= t {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
