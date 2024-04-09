fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn main() {
    let mut a = 5;
    let mut b = 7;
    swap(&mut a, &mut b);
    println!("{},{}", a, b);
}
