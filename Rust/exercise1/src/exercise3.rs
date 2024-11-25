use std::ops;

#[derive(Copy, Clone)]
struct Complex{
    real_part: f32,
    imaginary_part: f32
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Self) -> Self::Output {
        Complex{
            real_part: self.real_part + other.real_part,
            imaginary_part: self.imaginary_part + other.imaginary_part
        }
    }
}

impl ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Self) -> Self::Output {
        Complex{
            real_part: self.real_part - other.real_part,
            imaginary_part: self.imaginary_part - other.imaginary_part
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Self) -> Self::Output {
        Complex {
            real_part: self.real_part * other.real_part - self.imaginary_part * other.imaginary_part,
            imaginary_part: self.real_part * other.imaginary_part + self.imaginary_part * other.real_part,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Complex;

    fn div(self, other: Self) -> Self::Output {
        let bottom_value = other.real_part * other.real_part + other.imaginary_part * other.imaginary_part;
        assert_ne!(bottom_value,0.0);
        Complex {
            real_part: (self.real_part * other.real_part + self.imaginary_part * other.imaginary_part) / bottom_value,
            imaginary_part: (self.imaginary_part * other.real_part - self.real_part * other.imaginary_part) / bottom_value,
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.imaginary_part > 0.0{
            write!(f, "{} + {}j", self.real_part, self.imaginary_part)
        }
        else if self.imaginary_part < 0.0 {
            write!(f, "{} - {}j", self.real_part, self.imaginary_part * -1.0)
        }
        else {
            write!(f, "{}", self.real_part)
        }
    }
}

fn main()
{
    let first_number: Complex  = Complex{
        real_part: 10.0,
        imaginary_part: 10.0
    };
    let second_number: Complex  = Complex{
        real_part: 20.0,
        imaginary_part: 20.0
    };
    let sum = first_number + second_number;
    let sub = first_number - second_number;
    let mul = first_number * second_number;
    let div = first_number / second_number;
    println!("Sum : {}", sum);
    println!("Sub : {}", sub);
    println!("Mul : {}", mul);
    println!("Div : {}", div);
}
