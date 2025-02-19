use proconio::input;

fn main() {
    input! {
        q: usize,
        querys: [(u8,usize); q]
    }

    let mut a = vec![];
    for &(t, x) in querys.iter() {
        match t {
            1 => {
                a.push(x);
            }
            2 => {
                let n = a.len();
                println!("{}", a[n - x]);
            }
            _ => {}
        }
    }
}
