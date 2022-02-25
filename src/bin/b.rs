use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        ts: [i32; n],
    }

    let mut map = HashMap::<i32, i32>::new();

    ts.into_iter()
      .for_each(|t| *map.entry(t).or_default() += 1);

    println!("{}", map.len());
}
