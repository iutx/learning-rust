fn main() {
    /*
    Declare Variable
    */
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
    println!("The const value is {}", MAX_POINTS);

    /*
    Variable Type
    */
    // int, uint: 8,16,32,64,arch(isize, usize: depending on arch)
    let i1: u8 = 100;
    let i2: i8 = 100;
    println!("i8: {}, u8: {}", i1, i2);

    let i3: isize = 100;
    let i4: usize = 100;
    println!("isize: {}, usize: {}", i3, i4);

    // read more easily, means 1000
    let i5 = 1_000;
    println!("Read more easily {}", i5);

    // 0xff -> 255
    println!("0xff is  {}", 0xff);

    // bytes A->65
    println!("Bytes A is {}", b'A');

    // [WARNING] integer overflow: can't impl in cargo 1.63.0 version.
    // u8 range [0,255], if set value 256
    // In debug mode, will panic: the literal `256` does not fit into the type `u8` whose range is `0..=255`
    // In release mode, will skip panic, value 256 will overflow to min in range, result is 0, 257 result is 1.
    // let overflow:u8 = 256;
    // println!("u8 overflow, result is {}", overflow)


    // tuple type
    let tup: (i8, u8, f64) = (-10, 10, 1.0);
    println!("Tuple index 0 is {}", tup.0);
    println!("Tuple index 1 is {}", tup.1);
    println!("Tuple index 2 is {}", tup.2);
    // out of index, no field `3` on type `(i8, u8, f64)`
    // println!("tuple index 3 is {}", tup.3);

    // destructuring
    let (x, y, z) = tup;
    println!("Destructuring from tuple, value x is {}", x);
    println!("Destructuring from tuple, value y is {}", y);
    println!("Destructuring from tuple, value z is {}", z);

    // array type
    let array = [1, 3, 5, 7, 9];
    println!("Array index 3 is {}", array[3]);

    // initialize array, equal [10, 10, 10]
    let init = [10; 3];
    println!("Array initialize index 0 is {}", init[0])

    // array type is static, dynamic array type is `vector`.
}
