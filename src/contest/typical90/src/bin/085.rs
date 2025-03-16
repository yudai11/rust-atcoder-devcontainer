use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let mut ans = 0_usize;

    let mut factors = vec![];

    // 約数の列挙
    let l = k.sqrt();
    for i in 1..=l {
        if k % i == 0 {
            factors.push(i);
        }
    }

    let m = factors.len();

    // (a,b,c) の個数を列挙
    for i in 0..m {
        // if factors[i] * factors[i] * factors[i] > k {
        //     break;
        // }
        for j in i..m {
            if k / factors[i] / factors[j] < factors[j] {
                break;
            }
            if k % (factors[i] * factors[j]) == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
