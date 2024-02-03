fn main() {
    base_type();
    string_type();
    tuple_type();
    struct_type();
    enum_type();
    array_type()
}

fn base_type() {
    println!("------ base type ------");
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
}

fn string_type() {
    println!("------ string type ------");

    let str_cannot_change = "hello";
    // can't change str_cannot_change
    // str_cannot_change.push_str(" world");
    println!("str_cannot_change is {}", str_cannot_change);

    // string function
    let str_replace = String::from("hello rust");
    let new_str = str_replace.replace("rust", "Rust");
    println!("[replace] Replace string is {}", new_str);

    let str_replace_2 = String::from("hello rust, i love rust");
    let new_str_2 = str_replace_2.replacen("rust", "Rust", 1);
    println!("[replacen] Replace string is {}", new_str_2);

    let mut str_replace_3 = String::from("hello rust, i love rust");
    str_replace_3.replace_range(6..10, "Rust");
    println!("[replace_range] Replace string is {}", str_replace_3);

    // pop: remove last char and return it
    let mut str_pop = String::from("hello rust");
    let str_pop_last_char = str_pop.pop();
    println!("pop last char is {}", str_pop_last_char.unwrap());

    // remove: remove specified range and return it
    let mut str_remove = String::from("hello rust");
    let str_remove_range = str_remove.remove(6);
    println!("remove range is {}", str_remove_range);

    // truncate: remove all char from specified index
    let mut str_truncate = String::from("hello rust");
    str_truncate.truncate(5);
    // print: hello
    println!("truncate string is {}", str_truncate);

    // clear: remove all char
    let mut str_clear = String::from("hello rust");
    str_clear.clear();
    println!("clear string len is {}", str_clear.len());

    // string concatenate: + operator
    let str_con_append = String::from("hello ");
    let str_con_rust = String::from("rust");
    // why must use &str_con_rust ?
    // use + operator, equal to call `add` function in package `std::string`
    // must use &str_con_rust, because `add` function need reference type.
    let str_con_result = str_con_append + &str_con_rust;
    // build failed, because str_con_append is moved to str_con_result.
    // println!("append str is {}", str_con_append);
    println!("concatenate string is {}", str_con_result);

    // string concatenate: format! macro
    let str_format_append = String::from("hello ");
    let str_format_rust = String::from("rust");
    let str_format_result = format!("{}{}", str_format_append, str_format_rust);
    println!("format string is {}", str_format_result);

    let byte_escape = String::from("I'm writing \x52\x75\x73\x74!");
    println!("byte escape is {}", byte_escape);
    // don't use escape, use raw string
    let raw_escape = r"I'm writing \x52\x75\x73\x74!";
    println!("raw escape is {}", raw_escape);

    // or add escape char
    let raw_escape_2 = "I'm writing \\x52\\x75\\x73\\x74!";
    println!("raw escape is {}", raw_escape_2);

    // if string contains "", use r#""# to wrap string
    let raw_escape_3 = r#"I'm writing "Rust"!"#;
    println!("raw escape is {}", raw_escape_3);

    // if still contains r#""#, use r##""## to wrap string, no limit.
    let raw_escape_4 = r##"I'm writing "Rust"!"##;
    println!("raw escape is {}", raw_escape_4);

    // operator utf-8
    let str_utf8 = String::from("你好世界");
    for i in str_utf8.chars() {
        println!("[utf-8] char is {}", i);
    }
    // iter bytes
    for i in str_utf8.bytes() {
        println!("[utf-8] byte is {}", i);
    }
    // get sub str from utf-8
    let str_utf8_sub = &str_utf8[0..3];
    // 0-3 is 你, 3-6 is 好, 6-9 is 世, 9-12 is 界
    // print: 你 and len 12
    println!("[utf-8] sub str is {}, total len: {}", str_utf8_sub, str_utf8.len());

    // use utf8_slice to get sub str
    let str_utf8_sub_2 = utf8_slice::slice(&str_utf8, 0, 3);
    // print: 你好世 and len 4
    println!("[utf-8] sub str is {}, total len: {}", str_utf8_sub_2, utf8_slice::len(&str_utf8));
}


fn tuple_type() {
    println!("------ tuple type ------");
    // tuple type
    let tuple_a: (i8, u8, f64) = (-10, 10, 1.0);
    println!("Tuple index 0 is {}", tuple_a.0);
    println!("Tuple index 1 is {}", tuple_a.1);
    println!("Tuple index 2 is {}", tuple_a.2);
    // out of index, no field `3` on type `(i8, u8, f64)`
    // println!("tuple index 3 is {}", tup.3);

    // destructuring
    let (x, y, z) = tuple_a;
    println!("Destructuring from tuple, value x is {}", x);
    println!("Destructuring from tuple, value y is {}", y);
    println!("Destructuring from tuple, value z is {}", z);

    // multi return value
    let (str, str_len) = calculate_length(String::from("hello"));
    println!("The length of '{}' is {}.", str, str_len);
}

fn calculate_length(s: String) -> (String, usize) {
    // must create new variable
    // if return (s, s.len()), will cause s moved to return value, can't use s after return.
    let len = s.len();
    (s, len)
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
}

fn struct_type() {

    // if you want to change field value, need add `mut` for instance.
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: "user1@email.com".to_string(),
    };
    user1.email = String::from("user1@email2.com");
    if user1.active {
        println!("[user1] user: {}, email: {}", user1.username, user1.email);
    }

    // build struct
    let new_user = build_user(user1.username, user1.email);
    println!("[new user] user: {}, email: {}", new_user.username, new_user.email);

    // update users
    // if you want to update some field, need use field init shorthand
    // tips: `..struct_instance` only can be used at last field
    let updated_user = User {
        active: false,
        ..new_user
    };
    println!("updated user is active: {}", updated_user.active);

    // new_user fields are moved to updated_user except active field
    println!("[after field move] new user is active: {}", new_user.active);
    // username already moved to updated_user, can't call it
    // println!("[after field move] new user username: {}", new_user.username);

    let updated_user_2 = User {
        active: new_user.active,
        ..updated_user
    };
    // in this case, active field is moved to updated_user_2, new_user.active is still valid
    // because type bool implement Copy trait, so it's value can be copied.
    println!("[after field move] new user active is {}, updated_user_2 active is {}",
             new_user.active, updated_user_2.active);

    // tuple struct:
    // - tuple struct is a struct without named fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _ = Color(0, 0, 0);
    let _ = Point(0, 0, 0);

    // unit-like struct:
    // - struct without any fields, similar to golang's struct{}
    struct UnitLikeStruct;
    let _ = UnitLikeStruct;

    // use `#[derive(Debug)]` to print struct
    // need add `#[derive(Debug)]` under struct definition
    println!("updated_user_2 is {:?}", updated_user_2);
    // more friendly print
    println!("updated_user_2 is {:#?}", updated_user_2);
    // if you want to custom print, need implement `fmt::Display` trait, not given case here.

    // dbg!
    // - will move ownership to dbg! macro, and print debug info, then return ownership.
    let dbg_user = User {
        username: dbg!(String::from("dbg_user")),
        ..updated_user_2
    };
    // will print debug info, e.g. file, line, column, etc.
    dbg!(&dbg_user);
}

fn build_user(username: String, email: String) -> User {
    // if params name is same with field name, can use field init shorthand
    User {
        active: true,
        username,
        email,
    }
}

fn enum_type() {
    println!("------ enum type ------");

    #[derive(Debug)]
    enum PokerSuit {
        Spade(u8),
        Heart(u8),
        Diamond(char),
        Club(char),
    }

    println!("Poker suit is {:?}", PokerSuit::Spade(8));
    println!("Poker suit is {:?}", PokerSuit::Heart(5));
    println!("Poker suit is {:?}", PokerSuit::Diamond('A'));
    println!("Poker suit is {:?}", PokerSuit::Club('K'));

    // enum field can be any type
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 255, 0);
    println!("Message quit is {:?}, write is {:?}, change color is {:?}", m1, m3, m4);
    // to read Move field, use `match` to match pattern
    match m2 {
        Message::Move { x, y } => println!("Read Move fields, x: {}, y: {}", x, y),
        _ => println!("No match pattern"),
    }

    // Rust drop null concept, use Option enum (included in prelude) to handle null, field is Some、None
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let five = Some(5);
    let plus_result = plus_one(five);
    let none = plus_one(None);
    println!("five plus one result is {:?}, None plus one result is {:?}", plus_result, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn array_type() {
    println!("------ array type ------");

    // In Rust have two types of arrays:
    // - fixed length: array, store in stack
    // - dynamic length: vector, store in heap

    // array type
    let array = [1, 3, 5, 7, 9];
    println!("Array index 3 is {}", array[3]);

    // initialize array, equal [10, 10, 10]
    let init = [10; 3];
    println!("Array initialize index 0 is {}", init[0]);

    // Why None-primitive type array can't initialize like this?
    // let str_array = [String::from("rust"); 4];
    // Primitive type already Copy trait, so it's value can be copied.
    // None-primitive type can't be copied, need create one by one.

    // None-primitive type member array, e.g. String
    let str_array: [String; 4] = [
        String::from("i love rust"),
        String::from("i love rust"),
        String::from("i love rust"),
        String::from("i love rust"),
    ];
    println!("String array index 2 is {}", str_array[2]);
    // Use inner method to initialize None-primitive array
    let str_array_2: [String; 8] = std::array::from_fn(|_i| String::from("i love rust"));
    println!("String array 2 index 2 is {}", str_array_2[2]);

    // Slice
    println!("Slice array is {:?}", &array[1..3]);
    let slice = &array[0..3];
    println!("Slice is {:?}", slice);

    // two-dimensional array
    let two_dimensional_array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    for i in two_dimensional_array {
        for j in i.iter() {
            println!("Two-dimensional array is {}", j);
        }
    }
}