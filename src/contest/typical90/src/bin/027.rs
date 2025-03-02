use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut ans = vec![];
    let mut seen = HashSet::new();
    for (i, &ref si) in s.iter().enumerate() {
        if seen.contains(&si) {
            continue;
        }
        seen.insert(si);
        ans.push(i + 1)
    }

    println!("{}", ans.iter().join("\n"));
}
