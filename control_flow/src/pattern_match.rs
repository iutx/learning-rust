#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub(crate) fn pattern_match() {
    // match similar to switch
    let coin = Coin::Penny;
    let val = value_in_cents(coin);
    println!("value of coin is {}, {}", value_in_cents(Coin::Quarter), val);

    // match expression can return value
    #[allow(dead_code)]
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1"
    };
    println!("ipv6 address is {}", ip);

    // match binding
    #[allow(dead_code)]
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColor(i32, i32, i32),
    }
    let action = [
        Action::Say(String::from("hello")),
        Action::MoveTo(10, 20),
        Action::ChangeColor(255, 255, 255),
    ];
    for act in &action {
        let result = match act {
            Action::Say(msg) => format!("say {}", msg),
            Action::MoveTo(x, y) => format!("move to {}, {}", x, y),
            Action::ChangeColor(r, g, b) => format!("change color to {}, {}, {}", r, g, b),
        };
        println!("action result is {}", result);
    }

    // if you only match one condition, use if let
    // Equivalent to:
    // match v {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    let v = Some(3);
    if let Some(3) = v {
        println!("three");
    }

    // matches! macro
    let foo = 'f';
    println!("foo is f? {}", matches!(foo, 'A'..='C' | 'a'..='z'));

    let bar = Some(4);
    println!("bar is 4? {}", matches!(bar, Some(x) if x > 3));

    // matches! filter in arrays
    let array = [1, 2, 3, 4, 5];
    let filtered_array: Vec<_> = array.iter().filter(|&&x| (3..=5).contains(&x)).collect();
    println!("filtered array is {:?}", filtered_array);

    // shadowing
    // Some(age) = age, age is shadowed by Some(age)
    // In none-primitive type, ownership will be moved
    // So use other var name, e.g. if let Some(x) = age
    let age = Some(30);
    println!("age is {:?}", age);
    if let Some(age) = age {
        println!("match age is {:?}", age);
    }
    println!("after match age is {:?}", age);

    // while let
    let mut stack = vec![1, 2, 3];
    // pop until stack is empty
    while let Some(top) = stack.pop() {
        println!("pop value is {}", top);
    }

    // Refutable: whether a pattern might fail to match
    // - e.g. let、for、match
    // Irrefutable: only match one condition, ignore others
    // - e.g. if let、while let
}

fn value_in_cents(coin: Coin) -> u8 {
    // match is exhaustive, must cover all possible value
    match coin {
        // branch condition not necessary to boolean
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // match content can have multiple line, needs to be wrapped in curly braces {}
        Coin::Quarter => {
            let num: u8 = 20;
            num + 5
        }
    }
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

pub(crate) fn pattern_match_cases() {
    // More cases
    let x = Some(5);
    let y = 10;

    match x {
        // x -> Some(5), doesn't match, skip
        Some(50) => println!("Got 50"),
        // Introducing new variable y, can match all Some() value
        // y is shadowed by Some(y), not the same as y = 10
        // matched
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    // Already leave match, ending shadow, y is still 10
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // or
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // range match
    // only support numeric or char
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    // struct match
    let point = Point { x: 1, y: 1 };
    // struct destructuring
    let Point { x: a, y: b } = point;
    println!("[struct match] point x is {}, y is {}", a, b);
    // struct destructuring
    match point {
        // y is 0, x is any value
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        // x is 0, y is any value
        Point { x: 0, y } => println!("on the y axis at {}", y),
        // x and y are any value
        Point { x, y } => println!("on neither axis: ({}, {})", x, y),
    }

    // enum destructuring
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // let msg = Message::Quit;
    // let msg = Message::Move { x: 10, y: 20 };
    // let msg = Message::ChangeColor(0, 0, 255);
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {} and in the y direction {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
    }

    // nested struct destructuring
    #[allow(dead_code)]
    enum Canvas {
        Write(String),
        SetColor(Color),
    }
    let canvas = Canvas::SetColor(Color::Rgb(0, 0, 255));
    match canvas {
        Canvas::Write(text) => println!("Text message: {}", text),
        Canvas::SetColor(Color::Rgb(r, g, b)) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
        Canvas::SetColor(Color::Hsv(h, s, v)) => println!("Change the color to hue {}, saturation {}, and value {}", h, s, v),
    }

    // array destructuring
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    println!("array destructuring, x is {}, y is {}", x, y);

    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        println!("array destructuring, x is {}", x);
    }
    if let [.., y] = arr {
        println!("array destructuring, y is {}", y);
    }

    // ignore match
    // _ can use in: match (similar to default), function parameter etc.
    let val_a = Some(50);
    let val_b = Some(5);
    match (val_a, val_b) {
        (Some(_), Some(5)) => println!("Got some value"),
        _ => println!("Default case, x = {:?}", val_a),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, last) => {
            println!("Some numbers: {},{},  {}", first, third, last);
        }
    }

    // _ can be used for ignore unused variable
    let _x = 10;
    // let _ diff let _x:
    // - let _x = 10; 10 will bind to _x event it's unused
    // - let _ = 10; 10 will be ignored, not bind to any variable
    // for example:
    let ignore_str = Some(String::from("hello"));
    if let Some(_ignore_str) = ignore_str {
        // ignore_str already moved to _ignore_str in if let
        // Primitive type is copy, so there was no issue in the previous `shadowing` example
        // println!("ignore_str is {:?}", ignore_str);
    }

    let ignore_str = Some(String::from("hello"));
    if let Some(_) = ignore_str {
        // ignore_str is still available
        // _ not move ownership
        println!("ignore_str is {:?}", ignore_str);
    }

    // use `..` ignore other fields
    let origin = Color::Rgb(0, 0, 255);
    match origin {
        Color::Rgb(r, ..) => println!("Change the color to red {}", r),
        _ => println!("Default case"),
    }
    // ignore middle part
    let arr = [1, 2, 3, 4, 5];
    match arr {
        [first, .., last] => {
            println!("first is {}, last is {}", first, last);
        }
    }

    // match guard
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // y not shadowed, still 10
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

#[allow(dead_code)]
enum Message {
    Content { id: i32 },
}

pub(crate) fn pattern_match_at() {
    // @ can be used to bind the value to a variable
    let msg = Message::Content { id: 5 };
    match msg {
        // @ will bind the matched value to the variable `id_variable`
        Message::Content { id: bind_id @ 3..=7 } => {
            println!("bind matched value {}", bind_id);
        }
        Message::Content { id: 10..=12 } => {
            println!("matched value is 10..=12");
        }
        Message::Content { id } => {
            println!("matched value is {}", id);
        }
    }

    // struct destructuring with @
    // you can use @ to bind to the new struct
    let p @ Point { .. } = Point { x: 0, y: 7 };
    println!("[struct match with @] point x is {}, y is {}", p.x, p.y);

    let point = Point { x: 0, y: 7 };
    if let p @ Point { x: _, y: 7 } = point {
        println!("[struct match with @] point is ({}, {})", p.x, p.y);
    }

    // after Rust 1.53
    match 1 {
        n @ 1..=3 => println!("1..=3, n is {}", n),
        n @ (7 | 8) => println!("(7 | 8), n is {}", n),
        _ => println!("default"),
    }
}
