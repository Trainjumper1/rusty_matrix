fn some_function(x: i32) -> i32 {
    let y = if x < 5 { x + 1 } else {x};
    y
}

fn main() {
    let y = {
        let x = 3;
        some_function(x)
    };

    println!("The value of y is: {y}");
}