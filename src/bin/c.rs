fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        lens: [(i32, i32); n],
    }

    let mut acc = false;

    for bits in 0..(1 << n) {
        let sub_list = (0..n).into_iter()
                             .map(|x| bits & (1 << x) != 0)
                             .collect::<Vec<bool>>();

        let ans = lens.iter()
                      .enumerate()
                      .map(|(index, &(a, b))| if sub_list[index] {a} else {b})
                      .sum::<i32>();

        acc = acc || ans == x;
    }

    println!("{}", if acc {"Yes"} else {"No"});
}
