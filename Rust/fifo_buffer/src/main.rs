use fifo_buffer;
fn main() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);
    fifo_buffer.add(4);
    fifo_buffer.add(5);
    fifo_buffer.add(6);
    let removed_value = fifo_buffer.remove().unwrap();
    let output =fifo_buffer.get_buffer();
    println!("Value removed from buffer : {}", removed_value);
    println!("Current state of buffer : {}", fifo_buffer);
    println!("Buffer: {:?}", output);

}
