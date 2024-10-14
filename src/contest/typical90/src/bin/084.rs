use proconio::{input, marker::Chars};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut ans: usize = 0;

    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut num_chr: [usize; 2] = [0; 2];

    if s[left] == 'o' {
        num_chr[0] += 1;
    } else {
        num_chr[1] += 1;
    }
    while right < n {
        while right < n && (num_chr[0] == 0 || num_chr[1] == 0) {
            right += 1;
            if right >= n {
                break;
            }
            if s[right] == 'o' {
                num_chr[0] += 1;
            } else {
                num_chr[1] += 1;
            }
        }

        while right < n && num_chr[0] > 0 && num_chr[1] > 0 {
            ans += n - right;
            if s[left] == 'o' {
                num_chr[0] -= 1;
            } else {
                num_chr[1] -= 1;
            }
            left += 1;
        }
    }

    println!("{:?}", ans);
}
