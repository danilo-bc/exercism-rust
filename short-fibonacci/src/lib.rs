/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![0 as u8;0]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0 as u8;count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fib_buf = create_buffer(5);
    for i in 0..=1{
        fib_buf[i] = 1;
    }
    for i in 2..5{
        fib_buf[i] = fib_buf[i-2] + fib_buf[i-1];
    }
    fib_buf
}
