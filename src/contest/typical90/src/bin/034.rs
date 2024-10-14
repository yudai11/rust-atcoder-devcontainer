use proconio::input;
// use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, k: usize,
        _a: [usize; n]
    }

    let a = press_coordinate(&_a);

    let mut list = vec![0; n];
    list[a[0]] += 1;

    let mut left = 0;
    let mut right = 0;
    let mut ans = 1;
    let mut cur_kind = 1;

    while right < n - 1 {
        while cur_kind <= k && right < n - 1 {
            right += 1;
            if list[a[right]] == 0 {
                cur_kind += 1;
            }
            list[a[right]] += 1;
            if cur_kind <= k {
                ans = ans.max(right - left + 1);
            }
        }
        while cur_kind > k {
            list[a[left]] -= 1;
            if list[a[left]] == 0 {
                cur_kind -= 1;
            }
            left += 1;
        }
    }

    println!("{}", ans);
}

fn press_coordinate(list: &Vec<usize>) -> Vec<usize> {
    let mut copy_set = HashSet::new();
    for &v in list {
        copy_set.insert(v);
    }

    let mut copy_list = vec![];
    for &x in copy_set.iter() {
        copy_list.push(x);
    }
    copy_list.sort();

    let mut press_map: HashMap<usize, usize> = HashMap::new();
    for (i, &k) in copy_list.iter().enumerate() {
        press_map.insert(k, i);
    }

    let mut pressed_list: Vec<usize> = vec![];
    for x in list {
        pressed_list.push(press_map[x]);
    }

    pressed_list
}
