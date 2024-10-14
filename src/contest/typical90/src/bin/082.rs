use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        l: u64, r: u64
    }
    const MOD: u64 = 1000000007;
    let ans = calcu_0_to_mod(l, r);

    println!("{}", ans % MOD);
}

fn calcu_0_to_mod(l: u64, r: u64) -> u64 {
    const MOD: u64 = 1000000007;
    let mut ans: u64 = 0;
    let mut j: u64 = 1;
    for i in 1..=19 {
        let vl: u64 = l.max(j);
        let vr: u64 = r.min(j * 10 - 1);
        if vl > vr {
            j *= 10;
            continue;
        }
        let val: u64 = (sub_1_to_x(vr) + MOD - sub_1_to_x(vl - 1)) % MOD;
        ans += i * val;
        ans %= MOD;
        if r < j * 10 {
            break;
        }
        j *= 10;

        // // If l and r have the same digit
        // if j <= l && r < j * 10 {
        //     if (l + r) % 2 == 0 {
        //         ans += ((l + r) / 2 % MOD) * (r + 1 - l) % MOD * i % MOD;
        //     } else {
        //         ans += (l + r) % MOD * ((r + 1 - l) / 2 % MOD) * i % MOD;
        //     }
        //     ans %= MOD;
        //     break;
        // }
        // if j <= l && l < j * 10 {
        //     if l % 2 == 0 {
        //         ans += ((10 * j - l) / 2 % MOD) * ((10 * j + l - 1) % MOD) * i % MOD;
        //     } else {
        //         ans += ((10 * j - l) % MOD) * ((10 * j + l - 1) / 2 % MOD) * i % MOD;
        //     }
        //     ans %= MOD;
        // }
        // if j > l && j * 10 < r {
        //     if j % 2 == 0 {
        //         ans += 9 * (j / 2 % MOD) * ((j * 11 - 1) % MOD) * i % MOD;
        //     } else {
        //         ans += 9 * (j % MOD) * ((j * 11 - 1) / 2 % MOD) * i % MOD;
        //     }
        //     ans %= MOD;
        // }
        // if j <= r && r < j * 10 {
        //     if (j + r) % 2 == 0 {
        //         ans += (r + 1 - j) % MOD * ((j + r) / 2 % MOD) * i % MOD;
        //     } else {
        //         ans += (r + 1 - j) / 2 % MOD * ((j + r) % MOD) * i % MOD;
        //     }
        //     ans %= MOD;
        //     break;
        // }

        // j *= 10;
    }
    // ans += 70;
    // ans %= MOD;

    // ans *= (n % MOD);
    // ans %= MOD;
    ans
}

fn sub_1_to_x(x: u64) -> u64 {
    const MOD: u64 = 1000000007;
    if x % 2 == 0 {
        let v1 = (x / 2) % MOD;
        let v2 = (x + 1) % MOD;
        return v1 * v2 % MOD;
    } else {
        let v1 = ((x + 1) / 2) % MOD;
        let v2 = x % MOD;
        return v1 * v2 % MOD;
    }
}
