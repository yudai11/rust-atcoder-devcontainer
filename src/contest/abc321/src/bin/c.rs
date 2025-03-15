use proconio::input;

fn main() {
    input! {
        k: usize
    }
    // 321-like number を全て入れる．
    let mut ans_set = vec![];
    for i in 1..2_usize.pow(10) {
        let mut res = 0_usize;
        let mut mult = 1_usize;
        for j in 0..10 {
            if (i >> j) & 1 != 0 {
                res += j * mult;
                mult *= 10;
            }
        }
        ans_set.push(res)
    }

    ans_set.sort();
    let ans = ans_set[k];
    println!("{}", ans);
}
