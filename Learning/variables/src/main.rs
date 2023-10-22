#[allow(unused_assignments)]
#[allow(unused_variables)]

fn plus_one(x: i32) -> i32 {
    return x + 1; // or barely x + 1 without semicolon
}

fn main() {
    let x = 4;
    println!("X + 1 = {}", plus_one(x));
    const PI: f32 = 3.1415;
    println!("PI is {}", PI);

}
