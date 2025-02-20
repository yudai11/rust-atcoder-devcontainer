use proconio::input;

fn main() {
    input! {
        _n: usize, k: usize,
        mut a: [usize;k]
    }

    a.sort();
    let mut ans = 0_usize;

    if k % 2 == 0 {
        for i in (0..k).step_by(2) {
            ans += a[i + 1] - a[i];
        }
    } else {
        let mut pre_ans = 0_usize;
        for i in (1..k).step_by(2) {
            pre_ans += a[i + 1] - a[i];
        }
        ans = pre_ans;
        for i in 1..k {
            if i % 2 == 1 {
                pre_ans += a[i + 1] - a[i - 1];
                pre_ans -= a[i + 1] - a[i];
                ans = ans.min(pre_ans);
            } else {
                pre_ans += a[i - 1] - a[i - 2];
                pre_ans -= a[i] - a[i - 2];
                ans = ans.min(pre_ans);
            }
        }
    }

    println!("{}", ans);
}
