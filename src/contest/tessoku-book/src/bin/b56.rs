use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize, q: usize,
        mut s: Chars,
        lr: [(Usize1,Usize1); q]
    }

    // 回文判定にローリングハッシュを使う。(Palindrome, Rolling hash)
    let stringhash_1 = StringHash::new(&s);
    s.reverse();
    let stringhash_2 = StringHash::new(&s);

    let mut ans = vec![];
    for &(l, r) in lr.iter() {
        if stringhash_1.hash_value(l, r) == stringhash_2.hash_value(n - r - 1, n - l - 1) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", ans.iter().join("\n"));
}

struct StringHash {
    r#mod: usize,
    power_base: Vec<usize>,
    h: Vec<usize>,
}

impl StringHash {
    fn new(s: &Vec<char>) -> Self {
        let n = s.len();
        let r#mod = 2_usize.pow(61) - 1;
        let mut t = vec![0_usize; n + 1];
        for i in 1..=n {
            t[i] = s[i - 1] as usize - 'a' as usize + 1;
        }
        let base = 88_u128;
        let mut power_base = vec![1_usize; n + 1];
        for i in 1..=n {
            power_base[i] = ((base * power_base[i - 1] as u128) % r#mod as u128) as usize;
        }
        let mut h = vec![0_usize; n + 1];
        for i in 1..=n {
            h[i] = ((base * h[i - 1] as u128 + t[i] as u128) % r#mod as u128) as usize;
        }

        return StringHash {
            r#mod,
            power_base,
            h,
        };
    }

    fn hash_value(&self, l: usize, r: usize) -> usize {
        let mut val = self.h[r + 1] as i128
            - ((self.h[l] as i128 * self.power_base[r - l + 1] as i128) % self.r#mod as i128);
        if val < 0 {
            val += self.r#mod as i128;
        }
        return val as usize;
    }
}
