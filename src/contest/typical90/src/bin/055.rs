use itertools::Itertools;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, p: usize, q: usize,
        a: [usize; n]
    }
    let mut ans = 0;

    for perm in (0..n).combinations(5) {
        let mut timed = 1;
        for i in 0..5 {
            timed *= a[perm[i]];
            timed %= p;
        }
        ans += if timed == q { 1 } else { 0 };
    }

    println!("{}", ans);
}
