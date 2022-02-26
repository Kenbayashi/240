fn main() {
    proconio::input! {
        n: usize,
        array: [i32; n],
    }

    let mut stack = Vec::<(i32, i32)>::new();

    for num in array {
        add_ball(&mut stack, num);

        if check(&stack) {
            let _ = stack.pop();
        }

        println!("{}", stack.iter().fold(0, |acc, &(_, count)| acc + count));
    }
}

fn add_ball(stack: &mut Vec<(i32, i32)>, next: i32) {
    if let Some((prev, count)) = stack.pop() {
        if prev == next {
            stack.push((prev, count + 1));
        } else {
            stack.push((prev, count));
            stack.push((next, 1));
        }
    } else {
        stack.push((next, 1));
    }
}

fn check(stack: &Vec<(i32, i32)>) -> bool {
    if let Some((num, count)) = stack.last() {
        num == count
    } else {
        false
    }
}
