use std::isize;

use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        xy: [(isize,isize);n]
    }

    // // 45 degree rotation
    // let mut xy: Vec<(isize, isize)> = vec![(0, 0); n];
    // for i in 0..n {
    //     let &(x, y) = &_xy[i];
    //     xy[i] = (x - y, x + y);
    // }

    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        x[i] = xy[i].0;
        y[i] = xy[i].1;
    }

    x.sort();
    y.sort();

    let (c, r) = (x[n / 2], y[n / 2]);
    let mut ans = 0;
    ans += x.iter().fold(0, |sum, &x| sum + (x - c).abs());
    ans += y.iter().fold(0, |sum, &y| sum + (y - r).abs());

    println!("{:?}", ans);
}
