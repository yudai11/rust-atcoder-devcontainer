use proconio::input;
use std::f64::consts::PI;
use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize,
        xy: [(f64,f64); n]
    }

    let mut ans: f64 = 0.0;

    for i in 0..n {
        let (x, y) = xy[i];
        let mut angle_list: Vec<f64> = vec![];
        for j in 0..n {
            if j == i {
                continue;
            }
            let (z, w) = xy[j];
            // arctan と度数法への変換(0除算のケアもOK)
            angle_list.push(((z - x).atan2(w - y)) * 180.0 / PI % 360.0);
        }

        // f64型のsortはpartial orderを用いる．実数のソート
        angle_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for j in 0..n - 1 {
            let x = angle_list[j] + 360.0;
            angle_list.push(x);
        }

        for &aj in angle_list.iter() {
            let s = aj + 180.0;
            // f64型のlower bound
            let ind = angle_list.lower_bound_by(|x| x.partial_cmp(&s).unwrap());
            if ind == 2 * n - 2 {
                let gap = s - angle_list[ind - 1];
                ans = ans.max(180.0 - gap);
            } else if ind == 0 {
                let gap = angle_list[ind] - s;
                ans = ans.max(180.0 - gap);
            } else {
                let gap = (s - angle_list[ind - 1]).min(angle_list[ind] - s);
                ans = ans.max(180.0 - gap);
            }
        }
    }

    println!("{}", ans);
}
