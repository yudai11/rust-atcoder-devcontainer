use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        m: usize,
        s: [Chars; 3]
    }

    let mut ans = 1000_000_000_usize;
    for i in 0..m {
        for j in 1..=m {
            if i + j >= ans {
                break;
            }
            for k in 1..=m {
                for l in (0..3).permutations(3) {
                    if s[l[0]][i % m] == s[l[1]][(i + j) % m]
                        && s[l[1]][(i + j) % m] == s[l[2]][(i + j + k) % m]
                    {
                        ans = ans.min(i + j + k);
                    }
                }
            }
        }
    }

    if ans < 1000_000 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
