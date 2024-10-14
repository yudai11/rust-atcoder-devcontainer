use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, k: usize,
        ab: [(usize,usize);n]
    }
    let mut scores: Vec<usize> = vec![];
    for i in 0..n {
        let (ai, bi) = ab[i];
        scores.push(bi);
        scores.push(ai - bi);
    }

    scores.sort();
    scores.reverse();
    let ans = scores[0..k].iter().sum::<usize>();

    println!("{}", ans);
}
