use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut x = s
        .iter()
        .enumerate()
        .map(|(i, a)| (a.iter().filter(|&&c| c == 'o').count(), i + 1))
        .collect::<Vec<(usize, usize)>>();

    x.sort_by(|&x, &y| {
        if x.0 != y.0 {
            // x.0 > y.0 となるように並び替え
            y.0.cmp(&x.0)
        } else {
            // x.1 < y.1 となるように並び替え
            x.1.cmp(&y.1)
        }
    });

    println!("{}", x.iter().map(|a| a.1).join(" "));
}
