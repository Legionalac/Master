struct Circle {
    r: f32,
}

struct Rectangle {
    a: f32,
    b: f32,
}

struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Circle {
    fn area(self) -> f32 {
        self.r * self.r * std::f32::consts::PI
    }
}

impl Rectangle {
    fn area(self) -> f32 {
        self.a * self.b
    }
}

impl Triangle {
    fn area(self) -> f32 {
        let s: f32 = (self.a + self.b + self.c) / 3.0;
        return (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt();
    }
}

fn main() {
    let a = Circle { r: 3.0 };
    let b = Rectangle { a: 3.0, b: 3.0 };
    let c = Triangle {
        a: 3.0,
        b: 6.0,
        c: 7.0,
    };

    println!("Circle area : {} ", a.area());
    println!("Rectangle area : {} ", b.area());
    println!("Triangle area : {} ", c.area());
}
