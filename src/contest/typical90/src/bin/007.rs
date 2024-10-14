use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        q: usize,
        b: [u64; q]
    }

    // let mut a: Vec<u64> = vec![0; n];
    // for i in 0..n {
    //     a[i] = _a[i];
    // }
    a.sort();

    // let near = a.binary_search(&10);
    // print!("{}", near as String);

    let mut ans: Vec<u64> = vec![0; q];
    for i in 0..q {
        let bi = b[i];
        let bins = a.binary_search(&bi);
        ans[i] = match bins {
            Ok(_) => 0,
            Err(x) => {
                if x == 0 {
                    a[0] - bi
                } else if 0 < x && x < n {
                    (a[x] - bi).min(bi - a[x - 1])
                } else {
                    bi - a[n]
                }
            }
        };
        // println!("{}", ans[i]);
    }

    for i in 0..q {
        println!("{}", ans[i]);
    }
}

fn bin_search(a: &Vec<u64>, n: usize, bi: u64) -> i64 {
    let mut lower: usize = 0;
    let mut upper: usize = n - 1;
    if a[upper] <= bi {
        return (bi - a[upper]) as i64;
    } else if bi <= a[0] {
        return (a[0] - bi) as i64;
    }
    loop {
        let test: usize = lower + (upper - lower) / 2;
        if test == lower {
            return (a[upper] - bi).min(bi - a[lower]).try_into().unwrap();
        }
        if a[test] <= bi {
            lower = test;
        } else {
            upper = test;
        }
    }
}
