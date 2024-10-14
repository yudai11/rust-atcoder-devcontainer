use num_integer::Roots;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize
    }

    let upper = n.sqrt() + 1;
    let mut x = n;
    let mut num_of_prime_factor: usize = 0;
    // let mut seen = vec![false; upper];
    for i in 2..upper {
        while x % i == 0 {
            num_of_prime_factor += 1;
            x /= i;
        }
    }

    if x > 1 {
        num_of_prime_factor += 1;
    }

    let mut i = 0;
    let mut pow_2 = 1;
    loop {
        if pow_2 >= num_of_prime_factor {
            println!("{i}");
            break;
        }
        pow_2 *= 2;
        i += 1;
    }
}
