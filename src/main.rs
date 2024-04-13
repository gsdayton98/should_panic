fn main() {
    println!("Hello, world! {}", sample_function(10));
}


fn sample_function(n: u32) -> u32 {
    if n > 42 {
        panic!("greater than 42");
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        assert_eq!(sample_function(10), 10);
    }

    #[test]
    #[should_panic]
    fn test_out_of_range() {
        _ = sample_function(666);
    }
}