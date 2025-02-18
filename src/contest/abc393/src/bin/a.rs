use proconio::input;

fn main() {
    input! {
        s1: String, s2: String
    }

    let d1 = s1 == "sick";
    let d2 = s2 == "sick";

    if d1 && d2 {
        println!("1");
    } else if d1 && !d2 {
        println!("2");
    } else if !d1 && d2 {
        println!("3");
    } else {
        println!("4");
    }
}
