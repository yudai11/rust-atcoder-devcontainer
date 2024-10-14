use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    const INF: usize = usize::MAX;

    println!("{:?}", a);
}

fn longest_increasing_sequence(a: &Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut lis_list: Vec<usize> = vec![];
    lis_list[0] = 0;
    for &i in &a {
        let mut idx: usize;
        match lis_list.binary_search(i) {
            Ok(index) => idx = index,
            Err(index) => idx = index,
        }
    }

    lis_list
}
