struct MyString {
    value: Vec<char>,
}

impl MyString {
    fn new() -> Self {
        Self { value: Vec::new() }
    }

    fn push(&mut self, char: char) {
        self.value.push(char);
    }

    fn len(&self) -> usize {
        return self.value.len();
    }
}

fn main() {
    let mut test = MyString::new();
    test.push('a');
    test.push('b');
    test.push('c');
    test.push('d');
    for c in test.value.iter() {
        println!("{}", c);
    }
    println!("{:?}", test.value);
    println!("Length of string is {}", test.len());
}
