use math_basic_ops;
use math_advanced_ops;
fn main() {
    let a = 10.0;
    let b = 2.0;

    println!("MATH BASIC OPS TEST");
    println!("Sum: {}", math_basic_ops::basic_ops::add(&a,&b));
    println!("Sub: {}", math_basic_ops::basic_ops::substract(&a,&b));
    println!("Mul: {}", math_basic_ops::basic_ops::multiply(&a,&b));
    println!("Div: {}", math_basic_ops::basic_ops::divide(&a,&b));

    println!("MATH ADVANCED OPS TEST");
    println!("Square: {}", math_advanced_ops::advanced_ops::square(&a));
}
