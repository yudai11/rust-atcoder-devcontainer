use proconio::{input, marker::Usize1};

const MOD: usize = 1000_000_007;

fn main() {
    input! {
        n: usize, q: usize,
        xyzw: [(Usize1,Usize1,Usize1,usize); q]
    }

    let mut ans = 1_usize;
    for i in 0..60 {
        // i桁目に注目する
        let mut ways = 0_usize;
        // Aのi桁目の0/1の組み合わせを全探索
        for j in 0..2_usize.pow(n as u32) {
            let mut feasi = true;
            for &(x, y, z, w) in xyzw.iter() {
                if ((j >> x) | (j >> y) | (j >> z)) & 1 != (w >> i) & 1 {
                    feasi = false;
                    break;
                }
            }
            if feasi {
                ways += 1;
            }
        }
        ans *= ways;
        ans %= MOD;
    }

    println!("{}", ans);
}
