use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        h: usize, w: usize
    }

    let ans: usize;

    if h == 1 || w == 1 {
        ans = h * w;
    } else {
        ans = ((h + 1) / 2) * ((w + 1) / 2);
    }

    println!("{}", ans);
}
