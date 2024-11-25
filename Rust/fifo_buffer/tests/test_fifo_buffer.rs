use fifo_buffer;

#[test]
fn test_add(){
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(6);

    assert_eq!([6, 0, 0, 0, 0], fifo_buffer.get_buffer());
}

#[test]
fn test_remove(){
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(6);
    let deleted = fifo_buffer.remove().unwrap();

    assert_eq!(6, deleted);
}

#[test]
fn test_add_and_remove_multiple() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);
    fifo_buffer.add(4);
    fifo_buffer.add(5);

    assert_eq!([1, 2, 3, 4, 5], fifo_buffer.get_buffer());

    let removed = fifo_buffer.remove().unwrap();
    assert_eq!(1, removed);
    let removed2 = fifo_buffer.remove().unwrap();
    assert_eq!(2, removed2);
    assert_eq!([3, 4, 5, 0, 0], fifo_buffer.get_buffer());
}

#[test]
fn test_overwrite_when_full() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 4>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);
    fifo_buffer.add(4);
    fifo_buffer.add(5);
    fifo_buffer.add(6); 
    fifo_buffer.add(7);
    assert_eq!([4, 5, 6, 7], fifo_buffer.get_buffer());
}

#[test]
fn test_buffer_display() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(10);
    fifo_buffer.add(20);
    fifo_buffer.add(30);

    let display_output = format!("{}", fifo_buffer);
    assert_eq!(display_output.trim(), "10 20 30");
}

#[test]
fn test_buffer_size_limits() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);
    fifo_buffer.add(4);
    fifo_buffer.add(5);
    fifo_buffer.add(6); 
    fifo_buffer.add(15);
    assert_eq!(fifo_buffer.get_buffer().len(), 5);

}

#[test]
fn test_multiple_remove() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 5>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);

    let removed1 = fifo_buffer.remove().unwrap();
    let removed2 = fifo_buffer.remove().unwrap();

    assert_eq!(removed1, 1);
    assert_eq!(removed2, 2);
    assert_eq!([3, 0, 0, 0, 0], fifo_buffer.get_buffer());
}


#[test]
fn test_remove_all_elements() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 3>::new();
    fifo_buffer.add(10);
    fifo_buffer.add(20);
    fifo_buffer.add(30);

    assert_eq!(fifo_buffer.remove().unwrap(), 10); 
    assert_eq!(fifo_buffer.remove().unwrap(), 20); 
    assert_eq!(fifo_buffer.remove().unwrap(), 30); 

    let result = fifo_buffer.remove();
    assert!(result.is_err()); 
    assert_eq!(result.unwrap_err(), "There is no value in buffer");
}


#[test]
fn test_empty_buffer() {
    let fifo_buffer = fifo_buffer::FifoBuffer::<i32, 150>::new();
    assert_eq!(fifo_buffer.get_buffer(), [0; 150]); 
}

#[test]
fn test_full_buffer() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 3>::new();
    fifo_buffer.add(1);
    fifo_buffer.add(2);
    fifo_buffer.add(3);
    fifo_buffer.remove().unwrap(); 
    fifo_buffer.remove().unwrap(); 
    fifo_buffer.add(4); 
    fifo_buffer.add(5);
    assert_eq!([3, 4, 5], fifo_buffer.get_buffer());
}

#[test]
fn test_add_and_remove(){
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<i32, 4>::new();
    let result = fifo_buffer.remove();
    assert!(result.is_err()); 
    let result = fifo_buffer.remove();
    assert!(result.is_err()); 
    assert_eq!(result.unwrap_err(), "There is no value in buffer");
    fifo_buffer.add(3);
    assert_eq!([3, 0, 0, 0], fifo_buffer.get_buffer());
    let removed1 = fifo_buffer.remove().unwrap();
    assert_eq!(removed1, 3);    
    let result = fifo_buffer.remove();
    assert!(result.is_err()); 
    assert_eq!(result.unwrap_err(), "There is no value in buffer");

}

#[test]
fn test_mixed_operations_with_float_buffer() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<f32, 4>::new();

    // Add some elements
    fifo_buffer.add(1.1);
    fifo_buffer.add(2.2);

    // Remove one element
    let removed = fifo_buffer.remove().unwrap();
    assert_eq!(removed, 1.1);

    // Add more elements
    fifo_buffer.add(3.3);
    fifo_buffer.add(4.4);
    fifo_buffer.add(5.5);

    // Validate the buffer content after wraparound
    assert_eq!([2.2, 3.3, 4.4, 5.5], fifo_buffer.get_buffer());
}

#[test]
fn test_mixed_operations_with_char_buffer() {
    let mut fifo_buffer = fifo_buffer::FifoBuffer::<char, 4>::new();

    // Add some elements
    fifo_buffer.add('m');
    fifo_buffer.add('n');

    // Remove one element
    let removed = fifo_buffer.remove().unwrap();
    assert_eq!(removed, 'm');

    // Add more elements
    fifo_buffer.add('o');
    fifo_buffer.add('p');
    fifo_buffer.add('q');

    // Validate the buffer content after wraparound
    assert_eq!(['n', 'o', 'p', 'q'], fifo_buffer.get_buffer());
}