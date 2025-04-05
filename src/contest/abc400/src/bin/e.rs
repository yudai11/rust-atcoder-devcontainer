use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        q: usize,
        aa: [usize; q]
    }

    let max_val = aa.iter().fold(0_usize, |res, &x| res.max(x));
    // 候補となる素数の上界
    let n = (max_val / 4).sqrt() + 1;

    // n以下の素数を格納 O(n / log(n))
    let mut primes = vec![];
    let mut sieve = vec![false; n + 1];
    for i in 2..=n {
        if sieve[i] {
            continue;
        }
        primes.push(i);
        let mut j = i;
        while j <= n {
            sieve[j] = true;
            j += i;
        }
    }

    let m = primes.len();

    let l = max_val.sqrt() + 1;
    let mut pre_ans = vec![0_usize; l + 1];
    for i in 0..m - 1 {
        for j in i + 1..m {
            let pi = primes[i];
            let pj = primes[j];
            if pi * pj >= l {
                break;
            }
            let mut res_i = 1_usize;
            for _x in 0..100 {
                res_i *= pi;
                if res_i >= l {
                    break;
                }
                let mut test = res_i;
                for _y in 0..100 {
                    test *= pj;
                    if test >= l {
                        break;
                    }
                    pre_ans[test] = test;
                }
            }
        }
    }

    for i in 0..l {
        pre_ans[i + 1] = pre_ans[i + 1].max(pre_ans[i]);
    }

    let ans = aa
        .iter()
        .map(|&x| pre_ans[x.sqrt()])
        .collect::<Vec<usize>>();

    // let mut ans = vec![];
    // for &ai in aa.iter() {
    //     let x = ai.sqrt();
    //     ans.push(pre_ans[x] * pre_ans[x]);
    // }

    println!("{}", ans.iter().map(|&x| x * x).join("\n"))
}
