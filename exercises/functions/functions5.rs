// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// DONE

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    let ret = num * num;
    ret
}
