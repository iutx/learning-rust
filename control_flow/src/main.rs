fn main() {
    branch_flow_if();
    loop_flow_loop();
    loop_flow_while();
    loop_flow_for();
    pattern_match()
}

fn branch_flow_if() {
    // if flow control
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // expected `bool`, found integer
    // condition must explicit declaration bool
    // if number {
    //     println!("condition must be bool type");
    // }

    if number != 0 {
        println!("condition must be bool");
    }

    if number <= 0 {
        println!("number is less than 0");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("greet")
    }

    let condition = true;
    // must return same type value
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is {}", number);
}

fn loop_flow_loop() {
    // loop
    let mut count = 0;
    println!("start loop echo, target count is 10");
    loop {
        if count == 10 {
            break;
        }
        println!("again, current count is {}", count);
        count += 1;
    }

    // loop break with value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break and execute counter * 2;
            // break can carry value, similar to return
            break counter * 2;
        }
    };

    println!("The values of result is {}", result);
}

fn loop_flow_while() {
    // while
    let mut number = 3;
    while number != 0 {
        println!("current number value is {}", number);
        number = number - 1;
    }
}

fn loop_flow_for() {
    // for loop with range
    let values = [10, 20, 30, 40, 50];
    // Equivalent to: for item in IntoIterator::into_iter(collection)
    for val in &values {
        println!("value from tuple is {}", val)
    }
    // primitive type is copy, so values[0] is still available
    println!("after for loop, values (index 0) is {:?}", values[0]);

    // if you want to change val in loop, use `&mut`
    let mut mut_values = [10, 20, 30, 40, 50];
    // Equivalent to: for item in mut_values.iter_mut()
    for item in &mut mut_values {
        *item = *item + 1;
    }
    println!("mut array after for loop, values (index 0) changed to is {:?}", mut_values[0]);

    // ownership move
    let str_array: [String; 4] = std::array::from_fn(|_i| String::from("hello"));
    for item in str_array {
        println!("value from str_array is {}", item)
    }
    // String doesn't implement Copy trait, so str_array[0] is moved
    // println!("after for loop, str_array (index 0) is {:?}", str_array[0]);

    // iter generator
    for i in (1..4).rev() {
        println!("value from (1..4).rev(), item is {}", i)
    }

    // for with index and value
    for (i, item) in (1..4).enumerate() {
        println!("value from (1..4).enumerate(), index is {}, item is {}", i, item)
    }

    // specified for times
    for i in 1..=5 {
        println!("value from 1..=5, item is {}", i)
    }
}

fn pattern_match() {

}