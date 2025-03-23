use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        _n: usize, q: usize,
        s: Chars,
        abcd: [(Usize1,Usize1,Usize1,Usize1); q]
    }

    let string_hash = StringHash::new(&s);
    let mut ans = vec![];
    for &(a, b, c, d) in abcd.iter() {
        if string_hash.hash_value(a, b) == string_hash.hash_value(c, d) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", ans.iter().join("\n"));
}

// ローリングハッシュ　(Rolling Hash)
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
