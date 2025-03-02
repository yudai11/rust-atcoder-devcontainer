use itertools::Itertools;
use proconio::input;
use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        b: [usize;q]
    }

    a.sort();
    let mut ans = vec![0_usize; q];

    for (i, &bi) in b.iter().enumerate() {
        let ind = a.lower_bound(&bi);
        if ind == n {
            ans[i] = bi - a[n - 1];
        } else if ind == 0 {
            ans[i] = a[0] - bi;
        } else {
            ans[i] = (bi - a[ind - 1]).min(a[ind] - bi);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
