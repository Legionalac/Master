mod math;

fn main() {
    let a = 10;
    let b = 5;
    println!("Sum : {}", math::add(&a, &b));
    println!("Sub : {}", math::subscract(&a, &b));
}
