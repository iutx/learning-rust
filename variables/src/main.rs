fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    // cannot assign twice to immutable variable
    // x = 6;
    // println!("The value of x is {}", x);

    // let can afresh variable existed
    // shadow existed variable.
    let x = x + 1;
    println!("The new value of x is {}", x);

    // let also can change variable type
    let x = "i'm string type";
    println!("The new value of x is '{}', length is {}, new type is string", x, x.len());

    // mut means variable is mutable.
    let mut y = 5;
    println!("The value of y is {}", y);

    y = 10;
    println!("The new value of y is {}", y);

    // mut only means variable is mutable,
    // can change value, can't change type. change type use let to overwrite variable.
    // y = "hello";

    // constant is different with immutable variable
    const MAX_POINTS: u32 = 100_000;
    println!("The const value is {}", MAX_POINTS)
}
