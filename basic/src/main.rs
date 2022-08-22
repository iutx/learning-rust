use std::fmt::format;

fn main() {
    let hello = "Hello";
    let world = "World";
    // use {} to interpolate
    println!("{}, {}!", hello, world);

    // format strings
    let new_world = format!("{}, World!", hello);
    println!("{}", new_world);

    // panic!("crash and burn");

    let x = 1.1;
    let y = 2.2;
    // x = 2.2;  // cannot assign twice to immutable variable
    println!("x times y is {}", x * y);

    let mut z = 3.3; // use mut to make it mutable
    z = 4.4;
    let o: f64 = 5.5; // f64 is a type alias for float64, can be omitted
    // z = "Hello"; // change type is not allowed
    println!("z is {}, o is {}", z, o);

    // function
    let (ret1, ret2) = execute_cal(x, y);
    println!("x + y is {}, x * y is {}", ret1, ret2);
}

fn execute_cal(x: f64, y: f64) -> (f64, f64) {
    return (x + y, x * y);
}
