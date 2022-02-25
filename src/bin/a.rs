fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }

    let ans = b - a == 1 || b - a == 9;
    println!("{}", if ans {"Yes"} else {"No"});
}
