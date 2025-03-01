use proconio::input;

fn main() {
    input! {
        n: usize, x: usize,
        mut ud: [(usize,usize);n]
    }

    let mut ans = 0_usize;

    for i in 1..n {
        if ud[i].0 > ud[i - 1].0 + x {
            let gap = ud[i].0 - ud[i - 1].0 - x;
            ans += gap;
            ud[i].0 -= gap;
        }
        let mut j = i;
        while j > 0 && ud[j - 1].0 > ud[j].0 + x {
            let gap = ud[j - 1].0 - ud[j].0 - x;
            ans += gap;
            ud[j - 1].0 -= gap;
            j -= 1;
        }
    }

    // 二分探索
    let mut left = 0_usize;
    let mut right = ud.iter().fold(2000_000_000, |res, &(u, d)| res.min(u + d));
    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }

        if can(mid, &ud, n, x) {
            left = mid
        } else {
            right = mid
        }
    }

    ans += ud.iter().fold(0_usize, |sum, &(u, d)| sum + u + d - left);
    println!("{}", ans - n);
}

fn can(mid: usize, ud: &Vec<(usize, usize)>, n: usize, x: usize) -> bool {
    let mut new_u = vec![0_usize; n];
    for i in 0..n {
        let (u, d) = ud[i];
        let mut gap = u + d - mid;
        if d >= gap {
            new_u[i] = u;
        } else {
            gap -= d;
            new_u[i] = u - gap;
            if i == 0 {
                continue;
            }
            if new_u[i] > new_u[i - 1] + x || new_u[i - 1] > new_u[i] + x {
                return false;
            }
        }
    }

    return true;
}
