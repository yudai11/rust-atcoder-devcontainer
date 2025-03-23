use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    let n = s.len();

    let stringhash_1 = StringHash::new(&s);
    s.reverse();
    let stringhash_2 = StringHash::new(&s);

    // 回分の開始位置
    let mut k = 0_usize;
    loop {
        if stringhash_1.hash_value(k, n - 1) == stringhash_2.hash_value(0, n - 1 - k) {
            break;
        }
        k += 1;
    }

    s.reverse();
    let mut t = s.clone();

    for i in (0..k).rev() {
        t.push(s[i]);
    }

    println!("{}", t.iter().join(""));
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
            t[i] = s[i - 1] as usize - 'A' as usize + 1;
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
