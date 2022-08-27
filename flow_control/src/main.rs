fn main() {
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
            break counter * 2;
        }
    };

    println!("The values of result is {}", result);


    // while
    let mut number = 3;
    while number != 0 {
        println!("current number value is {}", number);
        number = number - 1;
    }

    // for
    let values = [10, 20, 30, 40, 50];
    for value in values {
        println!("value from tuple is {}", value)
    }

    // iter generator
    for i in (1..4).rev() {
        println!("value from (1..4).rev(), item is {}", i)
    }
}
