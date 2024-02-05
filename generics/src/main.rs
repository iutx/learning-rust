fn main() {
    hello_generic();
    struct_generic();
    struct_method_generic();
    const_generic();

    // Rust will generate all types of the generic function at compile time, named monomorphization.
    // So, the performance of the generic function is the same as the specific type function.
}

fn hello_generic() {
    let added_num = add(1, 2);
    let added_float = add(1.0, 1.1);
    println!("Added number: {} Added float: {}", added_num, added_float);
}

// Use Trait to limit the type of T
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
#[allow(dead_code)]
struct MultiTypePoint<T, U> {
    x: T,
    y: U,
}

fn struct_generic() {
    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.0, y: 2.0 };
    let multi_type_point = MultiTypePoint { x: 1, y: 2.0 };
    println!("int point: {:?} float point: {:?}, multi type point: {:?}",
             int_point, float_point, multi_type_point);

    // There are some cases in Rust, e.g. Options、Result, that use generics.
}

// impl<T> defined the method for the generic struct
// Point<T> means struct type, not means defined type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> MultiTypePoint<T, U> {
    // <T, U> is struct type
    // <V, W> is new define type for the method
    fn mix_up<V, W>(self, other: MultiTypePoint<V, W>) -> MultiTypePoint<T, W> {
        MultiTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<i32> {
    fn method_for_i32(&self) -> i32 {
        self.x
    }
}

fn struct_method_generic() {
    let _float_point = Point { x: 1.0, y: 2.0 };
    let int_point = Point { x: 1, y: 2 };
    println!("int point x: {}", int_point.x());

    let multi_type_point = MultiTypePoint { x: 1, y: 2.0 };
    let mixed = multi_type_point.mix_up(MultiTypePoint { x: "hello", y: 1 });
    println!("mixed: {:?}", mixed);

    let _ = int_point.method_for_i32();
    // This will cause compile error
    // method not found in `Point<{float}>`
    // let _ = float_point.method_for_i32();
}

// After Rust 1.51, we can use const generic
fn const_generic() {
    // [i32; 2] and [i32; 3] is different type in Rust.
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

#[allow(dead_code)]
fn generic_const_expr<T>(_val: T) {
    // TODO
}