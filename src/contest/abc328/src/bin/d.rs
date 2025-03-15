use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    // let mut ans = VecDeque::new();
    let mut ans = vec![];
    let l = s.len();

    let mut i = 0_usize;
    while i < l {
        if ans.len() > 1 && s[i] == 'C' {
            let z = ans.pop().unwrap();
            let y = ans.pop().unwrap();
            if y != 'A' || z != 'B' {
                ans.push(y);
                ans.push(z);
                ans.push(s[i]);
            }
        } else {
            ans.push(s[i]);
        }
        i += 1;
    }

    if ans.len() > 0 {
        println!("{}", ans.iter().join(""));
    }
}
