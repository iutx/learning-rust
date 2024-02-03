fn main() {
    // : means call from function belong String namespace
    // String type, if use `let mut s = "hello"`, type s is &str
    // String type will allocate memory space in the heap
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("The value of s is {}", s);

    // String type (mutable, dynamic size) will allocate unknown size memory space in heap
    // Allocate memory in program running
    // Need give back memory when variable use up
    // (1. GC, e.g. Golang 2. free by developer, e.g. C++ )
    // In Rust, memory will give back when variable out of action scope
    scope_demo();

    // multi variables bind
    // i32 is fixed and known size, will push the stack at the same time
    let x = 5;
    // x will copy value to y
    let y = x;
    println!("The value of x is {}, y is {}", x, y);

    // move
    var_move();

    // deep copy
    deep_copy();

    // none move
    none_move();

    // ownership in function
    ownership_fn();

    // return and scope
    return_scope();

    // reference
    reference();

    // dangle pointer
    dangle_show();

    // slice
    slice();
}

fn scope_demo() {
    // variable s defined, type is String
    let mut s = String::from("hello");
    // operation with s
    s.push_str(", world!");
} // scope end, s lose. Rust will call function drop() automatic, memory will free

fn var_move() {
    // But same defined in String type.
    //  s1
    // +------+------+
    // | name |value |
    // +------+------+
    // | ptr  |      |-------+         +------+------+
    // +------+------+       |         |index |value |
    // | len  |  5   |       |         +------+------+
    // +------+------+       +-------> |  0   |  h   |
    // | cap  |  5   |       |         +------+------+
    // +------+------+       |         |  1   |  e   |
    //                       |         +------+------+
    //  s2                   |         |  2   |  l   |
    // +------+------+       |         +------+------+
    // | name |value |       |         |  3   |  l   |
    // +------+------+       |         +------+------+
    // | ptr  |      |-------+         |  4   |  o   |
    // +------+------+                 +------+------+
    // | len  |  5   |
    // +------+------+
    // | cap  |  5   |
    // +------+------+
    let s1 = String::from("hello");
    let s2 = s1;
    // when scope end, s1 and s2 will free repeat, cause memory error: second release
    // In Rust let s2 = s1; s1 will abandon
    // Run will cause `value borrowed here after move`
    // println!("The value of s1 is {}", s1)
    println!("The value of s1 is moved to s2, use s1 will cause `value borrowed here after move`");
    // Not shallow copy, not deep copy, in Rust: move
    //  s1(move)
    // +------+------+
    // | name |value |
    // +------+------+
    // | ptr  |      |                 +------+------+
    // +------+------+                 |index |value |
    // | len  |  5   |                 +------+------+
    // +------+------+       +-------> |  0   |  h   |
    // | cap  |  5   |       |         +------+------+
    // +------+------+       |         |  1   |  e   |
    //                       |         +------+------+
    //  s2                   |         |  2   |  l   |
    // +------+------+       |         +------+------+
    // | name |value |       |         |  3   |  l   |
    // +------+------+       |         +------+------+
    // | ptr  |      |-------+         |  4   |  o   |
    // +------+------+                 +------+------+
    // | len  |  5   |
    // +------+------+
    // | cap  |  5   |
    // +------+------+
    println!("The value s1 is moved, value of s2 is {}", s2);
    // Rust never create data deep copy automatic.
}

fn deep_copy() {
    // Rust never create data deep copy automatic.
    // Deep copy manual
    //  s1
    // +------+------+                 +------+------+
    // | name |value |                 |index |value |
    // +------+------+                 +------+------+
    // | ptr  |      |---------------->|  0   |  h   |
    // +------+------+                 +------+------+
    // | len  |  5   |                 |  1   |  e   |
    // +------+------+                 +------+------+
    // | cap  |  5   |                 |  2   |  l   |
    // +------+------+                 +------+------+
    //                                 |  3   |  l   |
    //                                 +------+------+
    //                                 |  4   |  o   |
    //                                 +------+------+
    //
    //  s2
    // +------+------+                 +------+------+
    // | name |value |                 |index |value |
    // +------+------+                 +------+------+
    // | ptr  |      |---------------->|  0   |  h   |
    // +------+------+                 +------+------+
    // | len  |  5   |                 |  1   |  e   |
    // +------+------+                 +------+------+
    // | cap  |  5   |                 |  2   |  l   |
    // +------+------+                 +------+------+
    //                                 |  3   |  l   |
    //                                 +------+------+
    //                                 |  4   |  o   |
    //                                 +------+------+
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("The value of s3 is {}, value of s4 clone from s3 is {}", s3, s4)
}

fn none_move() {
    let x = 5;
    let y = x;

    // none move happen
    println!("The value of x is {}, y is {}", x, y)

    // i32 is fixed and known size, will push the stack at the same time
    // in stack

    // Same in type which have trait named Copy, e.g.
    // All int type, e.g. u32
    // Bool, Char
    // All float type
    // Tuple extend item not Copy, e.g. (i32, i32) is Copy, (i32, String) is not Copy
}

fn ownership_fn() {
    let ownership_s = String::from("hello"); // ownership_s into scope
    ownership_fn1(ownership_s);
    // ownership_s lose scope, can't use again

    // println!("The value of ownership_s is {}", ownership_s) // value borrowed here after move
    println!("Variable ownership_s after move");

    let ownership_x = 5; // ownership_x into scope
    ownership_fn2(ownership_x); // nothing happen, ownership_x is Copy
    println!("The value ownership_x after call ownership_fn2, value is {}", ownership_x);
} // ownership_x lose scope, after is ownership_s, but s had moved, skip

fn ownership_fn1(str: String) { // str into scope
    println!("The value of param in ownership_fn1 is {}", str);
} // str scope lose, call drop automatic

fn ownership_fn2(num: i32) { // var num into scope
    println!("The value of param in ownership_fn2 is {}", num)
} // num scope lose, nothing happen


fn return_scope() {
    let _s1 = give_ownership(); // give_ownership() move return value to s1

    let s2 = String::from("hello"); // s2 into scope
    let _s3 = takes_and_gives_back(s2); // s2 move into function, scope lose

    // If you want to keep var ownership, you need make var as function return
}
// 1. s3 scope lose, drop
// 2. s2 have moved, skip
// 3. s1 scope lose last, drop


fn reference() {
    // use tuple to return multi values
    let param = String::from("hello");
    let (value, length) = return_multi_values(param);
    println!("The value is {}, length is {}", value, length);

    // reference to keep scope
    // &: you can use value when ownership not get.
    let s1 = String::from("hello");
    // &s1 allow create a pointer refer to s1 under the condition of lose scope
    //  s                        s1
    // +------+------+          +------+------+           +------+------+
    // | name |value |          | name |value |           |index |value |
    // +------+------+          +------+------+           +------+------+
    // | ptr  |      |--------->| ptr  |      |---------->|  0   |  h   |
    // +------+------+          +------+------+           +------+------+
    //                          | len  |  5   |           |  1   |  e   |
    //                          +------+------+           +------+------+
    //                          | cap  |  5   |           |  2   |  l   |
    //                          +------+------+           +------+------+
    //                                                    |  3   |  l   |
    //                                                    +------+------+
    //                                                    |  4   |  o   |
    //                                                    +------+------+
    let len = use_refer_keep_scope(&s1);
    println!("The value s1 is {}, length is {}", s1, len);


    // use mutable string
    let mut s2 = String::from("hello");
    let len = use_refer_keep_scope_mut(&mut s2);
    println!("The value of s2, changed value in function use_refer_keep_scope_mut, {}", len);

    // TODO: cannot borrow `s` as mutable more than once at a time
    let mut s3 = String::from("hello");
    let r1 = &mut s3;
    println!("The value borrow from s3, r1 is {}", r1);
    let r2 = &mut s3;
    println!("The value borrow from s3, r2 is {}", r2);
}

fn give_ownership() -> String {
    let some_string = String::from("here"); // some_string into scope
    some_string // return some_string, moved to receive
}

fn takes_and_gives_back(str: String) -> String {
    // str into scope
    str // str moved to receive
}

fn return_multi_values(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Use & to receive named: borrowing
fn use_refer_keep_scope(s: &String) -> usize { // s pointer to String
    // s.push_str(", world!"); because s1 is immutable, can't change value
    s.len()
} // s lose scope, because of s didn't handle ownership of refer to, skip, nothing happen


fn use_refer_keep_scope_mut(s: &mut String) -> usize {
    s.push_str(", world!");
    s.len()
}

fn dangle_show() {
    // Dangling pointer
    let reference_to_nothing = dangle();
    println!("The value of reference_to_nothing is {}", reference_to_nothing);
}

// expected named lifetime parameter
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn dangle() -> &String { // dangle return a reference to &String
//let s = String::from("hello"); // s bind to new String
// &s // s's reference return to caller
// } // s lose scope, destroyed in memory, danger!

fn dangle() -> String { // return String can solve problem, ownership will remove to caller
    let s = String::from("hello");
    s
}

// slice
fn slice() {
    let mut s = String::from("hello world!");

    let word = first_word_index(&s); // index result will bind to var world.

    s.clear(); // clear will reset String

    println!("{}", word);

    //  s
    // +------+------+           +------+------+
    // | name |value |           |index |value |
    // +------+------+           +------+------+
    // | ptr  |      |---------->|  0   |  h   |
    // +------+------+           +------+------+
    // | len  |  11  |           |  1   |  e   |
    // +------+------+           +------+------+
    // | cap  |  11  |           |  2   |  l   |
    // +------+------+           +------+------+
    //                           |  3   |  l   |
    //                           +------+------+
    //  world                    |  4   |  o   |
    // +------+------+           +------+------+
    // | name |value |           |  5   |      |
    // +------+------+           +------+------+
    // | ptr  |      |---------->|  6   |  w   |
    // +------+------+           +------+------+
    // | len  |  5   |           |  7   |  o   |
    // +------+------+           +------+------+
    //                           |  8   |  r   |
    //                           +------+------+
    //                           |  9   |  l   |
    //                           +------+------+
    //                           |  10  |  d   |
    //                           +------+------+
    let s = String::from("hello world");
    println!("slice 1 print: {}", &s[0..5]);
    println!("slice 1 same print: {}", &s[..5]);
    let len = s.len();
    println!("slice 2 print: {}", &s[6..11]);
    println!("slice 2 same print: {}", &s[6..len]);
    println!("slice 2 same print: {}", &s[6..]);
    println!("slice 3 same print: {}", &s[0..len]);
    println!("slice 3 same print: {}", &s[..]);

    let mut s = String::from("hello world");
    let result = first_word(&s);
    // s.clear(); &s: immutable borrow occurs here, result: immutable borrow later used here
    println!("first world of `{}` is {}", s, result)
}

fn first_word_index(s: &String) -> usize {
    // String -> bytes, &[u8]
    let bytes = s.as_bytes();

    // iter() can return item in turn.
    // enumerate() generate the iterator of reference item, receive index, item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}


