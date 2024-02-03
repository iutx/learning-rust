fn main() {
    println!(swap(&mut 1, &mut 2));
}

fn swap(x: &mut i32, y: &mut i32) {
    let t = *x;
    *x = *y;
    *y = t;
}
