use std::{
    fmt::{self, Display},
    usize,
};

pub struct FifoBuffer<T, const N: usize> {
    data: [T; N],
    size: usize,
    head: usize,
    tail: usize,
}

impl<T: Default + Copy, const N: usize> FifoBuffer<T, N> {
    pub fn new() -> Self {
        FifoBuffer {
            data: [T::default(); N],
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn add(&mut self, new_element: T) {
        self.data[self.tail % N] = new_element;
        self.tail += 1;
        self.size += 1;

        if self.size > N {
            self.size = N;
        }

        self.head = self.tail - self.size;
    }

    pub fn remove(&mut self) -> Result<T, &str> {
        if self.head >= self.tail {
            return Err("There is no value in buffer");
        }

        self.head += 1;
        self.size -= 1;
        return Ok(self.data[(self.head - 1) % N]);
    }

    pub fn get_buffer(&self) -> [T; N]
    {
        let mut output = [T::default(); N];
        let mut index = 0;

        for i in self.head..self.tail {
            output[index] = self.data[i%N];
            index+=1;
        }

        output
    }
    
}

impl<T: Display, const N: usize> fmt::Display for FifoBuffer<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.head..self.tail {
            write!(f, "{} ", self.data[i % N])?;
        }
        Ok(())
    }
}
