use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q]
    }
    let mut cum_sum_1_list = vec![0; n + 1];
    let mut cum_sum_2_list = vec![0; n + 1];
    let mut cum_sum_1 = 0;
    let mut cum_sum_2 = 0;

    for i in 1..=n {
        let cpi = cp[i - 1];
        if cpi.0 == 1 {
            cum_sum_1 += cpi.1;
        } else {
            cum_sum_2 += cpi.1;
        }
        cum_sum_1_list[i] = cum_sum_1;
        cum_sum_2_list[i] = cum_sum_2;
    }

    for i in 0..q {
        println!(
            "{} {}",
            cum_sum_1_list[lr[i].1] - cum_sum_1_list[lr[i].0 - 1],
            cum_sum_2_list[lr[i].1] - cum_sum_2_list[lr[i].0 - 1]
        );
    }
}
