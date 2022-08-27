fn main() {
    println!("Hello, world!");

    // demo function
    demo_function(100, 101);

    let x = {
        let y = 1;
        // if add ; `y + 1` will means statement
        // now means return values, and assign to x
        y + 1
    };

    println!("The value of x is {}", x);

    println!("function with return value {}", function_with_return());
    println!("function_with_return_add_1 return value {}", function_with_return_add_1(100))
}

fn demo_function(x: i32, y: i32) {
    println!("In function demo_function, params: x={}, y={}", x, y)
}

// -> return values
fn function_with_return() -> i32 {
    5
}

fn function_with_return_add_1(x: i32) -> i32 {
    x + 1
    // x + 1; will compile failed, error: expected `i32`, found `()`
    // x + 1; as a statement, default return a empty tuple ()
}